you can save and get objects from the cache file that will be created from the given name

main functions

Cache {

    pub async fn build(in_memory: bool, name: &str) -> Self 

    pub async fn save_obj<T>(&self, cache_name: &str, obj: &T) -> Result<(), CacheError>

    pub async fn save_obj_if_not_exist<T>(&self, cache_name: &str, obj: &T) -> Result<(), CacheError>

    pub async fn get_obj<T>(&self, cache_name: &str) -> Result<T, CacheError>

    pub async fn clear_cache(&self)
}