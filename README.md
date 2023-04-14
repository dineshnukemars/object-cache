<div><h1>Object Cache</h1>
<p> <a href="https://github.com/dineshnukemars/object-cache" target="_new"><img src="https://img.shields.io/badge/build-v0.2.1-green.svg" alt="Build Version"></a>
<a href="https://opensource.org/licenses/MIT" target="_new"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a> </p>
<ul>
    <li>A simple cache util that takes any object reference and caches it in memory or in a file.</li>
    <li>This crate uses Sqlite db to store objects.</li>
    <li>It converts the object into JSON format and stores it for the given key.</li>
    <li>The reverse happens when you retrieve the object.</li>
</ul>

<h2>Usage</h2>

<pre>
<code class="language-rust">
async fn sample(){
    // build Cache with name (Could be In memory or File based cache)
    let cache = Cache::build(true, "obj_cache").await;

    let data = TestStruct {
        name: "dinesh".to_owned(),
        email: "dinesh.nuke.mars@gmail.com".to_owned(),
        ph_no: 9999999999u64,
    };

    // pass key and object
    cache.save_obj("TestData", &data).await.unwrap();

    // visualize the data
    cache.pretty_print_all_cache();

    // get object anywhere in the project using key
    let cached_data: TestStruct = cache.get_obj("TestData").await.unwrap();
}
</code>
</pre>

<h2>Main Functions</h2>

<pre>
<code class="language-rust">
Cache {
    pub async fn build(in_memory: bool, cache_file_name: &str) -> Self 
    pub async fn save_obj&lt;T&gt(&self, key: &str, obj: &T) -&gt Result&lt;(), CacheError
    pub async fn save_obj_if_not_exist&lt;T&gt(&self, key: &str, obj: &T) -&gt Result&lt;(), CacheError
    pub async fn get_obj&lt;T&gt(&self, key: &str) -&gt Result&lt;T, CacheError
    pub async fn get_all_objs(&self) -&gt Result&lt;Vec&lt;CacheData&gt, CacheError&gt 
    pub async fn pretty_print_all_cache(&self) 
    pub async fn clear_cache(&self) 
}
</code>
</pre>

<p>For more detailed usage instructions, visit the <a href="https://github.com/dineshnukemars/object-cache" target="_new">GitHub repository</a>.</p>

<h2>Contributing</h2>

<p>
    We welcome contributions! Please see the <a href="https://github.com/yourusername/projectname/blob/main/CONTRIBUTING.md" target="_new"> contributing guidelines</a>
    for more information.
</p>

<h2>License</h2>

<p>This project is licensed under the terms of
    the <a href="https://opensource.org/licenses/MIT" target="_new">MIT License</a>. See
    the <a href="https://github.com/dineshnukemars/object-cache/blob/master/LICENSE" target="_new">LICENSE</a> file for more details.</p>
<hr>
<p>Made with &hearts; by <a href="mailto:dinesh.nuke.mars@gmail.com" target="_new">dinesh.nuke.mars@gmail.com</a>and <a
        href="https://github.com/dineshnukemars/object-cache/graphs/contributors" target="_new">contributors</a>.
</p>

</div>