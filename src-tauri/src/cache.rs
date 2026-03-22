use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;

/// 缓存条目，包含对应解析渲染后的文件展示路径及最终格式化内容
#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// 文件的展示名称（通常相对于项目根目录并处理过分隔符）
    pub display_path: String,
    /// 最终拼接并可能经过压缩（Minification）处理的内容字符串
    pub content: String,
}

/// 文件解析缓存管理，用于持久化存储或在同一会话中避免对同一文件的重复解析
/// 键由 (规范化绝对路径, 修改时间) 组成，确保内容的一致性
pub struct FileCache {
    /// 对应的键为 (abs_path, mtime)，值为 CacheEntry
    data: Mutex<HashMap<(PathBuf, SystemTime), CacheEntry>>,
}

impl FileCache {
    /// 创建一个新的空缓存实例
    pub fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }

    /// 根据绝对路径及文件修改时间获取缓存的条目
    ///
    /// # 参数
    /// * `abs_path` - 文件的规范化绝对路径，作为缓存键的主体部分
    /// * `mtime` - 文件的系统修改时间，用于检测内容变更
    pub fn get(&self, abs_path: &Path, mtime: SystemTime) -> Option<CacheEntry> {
        let key = (abs_path.to_path_buf(), mtime);
        self.data.lock().ok()?.get(&key).cloned()
    }

    /// 向缓存中更新或插入一个解析后的文件条目
    ///
    /// # 参数
    /// * `abs_path` - 绝对路径
    /// * `mtime` - 文件的系统修改时间
    /// * `display_path` - 格式化后的展示路径
    /// * `content` - 已解析并渲染完成的文件块内容
    pub fn set(&self, abs_path: PathBuf, mtime: SystemTime, display_path: String, content: String) {
        if let Ok(mut data) = self.data.lock() {
            data.insert((abs_path, mtime), CacheEntry { display_path, content });
        }
    }

    /// 清空所有已保存的解析记录
    pub fn clear(&self) {
        if let Ok(mut data) = self.data.lock() {
            data.clear();
        }
    }
}

impl Default for FileCache {
    fn default() -> Self {
        Self::new()
    }
}
