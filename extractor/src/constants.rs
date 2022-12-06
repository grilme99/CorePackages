use phf::phf_map;

/// Some packages are unlicensed and can be replaced with something else. Block them
/// entirely here.
pub const BANNED_PACKAGE_NAMES: [&str; 19] = [
    "Cryo",
    "Promise",
    "Otter",
    // Ban old versions of packages
    "LuauPolyfill-12e911c4-90b08185",
    "LuauPolyfill-2fca3173-0.4.2",
    "LuauPolyfill-2fca3173-0.3.4",
    "React-04005deb-0fbbfa70",
    "ReactDebugTools-04005deb-0fbbfa70",
    "ReactDevtoolsExtensions-04005deb-0fbbfa70",
    "ReactDevtoolsShared-04005deb-0fbbfa70",
    "ReactIs-04005deb-0fbbfa70",
    "ReactReconciler-04005deb-0fbbfa70",
    "ReactRoblox-04005deb-0fbbfa70",
    "RoactCompat-04005deb-0fbbfa70",
    "Scheduler-04005deb-0fbbfa70",
    "Shared-04005deb-0fbbfa70",
    "ReactProxy",
    "RoactProxy",
    "Roact17UpgradeFlag",
];

pub static DEPENDENCY_ALIASES: phf::Map<&'static str, &'static str> = phf_map! {
    "Promise" => "evaera/promise@4.0.0",
    "Cryo" => "freddylist/llama@1.1.1",
    "Otter" => "reselim/flipper@2.0.0",
    "ReactProxy" => "core-packages/react@17.0.1-rc.16",
    "RoactProxy" => "core-packages/roact-compat@17.0.1-rc.16.1",
    "LuauPolyfill-2fca3173-0.3.4" => "core-packages/luau-polyfill@1.1.0"
};

pub static PACKAGE_VERSION_OVERRIDES: phf::Map<&'static str, &'static str> = phf_map! {
    "RoactCompat-9c8468d8-8a7220fd" => "17.0.1-rc.16.1",
};

pub const MIT_LICENSE_PHRASES: [&str; 3] = [
    "licensed under the MIT license",
    "Copyright Node.js contributors. All rights reserved",
    "Copyright (c) 2013 Forbes Lindesay",
];

pub const APACHE_LICENSE_PHRASES: [&str; 1] = ["licensed under the Apache License, Version 2.0"];

// Some modules are so small that it's impossible to rewrite them enough to be considered unique.
// Explicitly allow those modules here.
pub const ALLOWED_MODULES: [&str; 7] = [
    "Collections/Collections/Map/init.lua",
    "Collections/Collections/init.lua",
    "Math/Math/clz32.lua",
    "ReactRoblox-9c8468d8-8a7220fd/ReactRoblox/ReactReconciler.roblox.lua",
    "InstanceOf/InstanceOf/init.lua",
    // Original source licensed under MIT
    "VirtualizedList/VirtualizedList/Lists/BidirectionalFlatList.lua",
    "VirtualizedList/VirtualizedList/jest.config.lua",
];

// Any module that needs to be rewritten should be included here
pub static SOURCE_REPLACEMENTS: phf::Map<&'static str, &'static str> = phf_map! {
    "Scheduler/getJestMatchers.roblox.lua" =>
        include_str!("../resources/sourceReplacements/getJestMatchers.roblox.lua"),
    "RoactCompat-9c8468d8-8a7220fd/RoactCompat/init.lua" =>
        include_str!("../resources/sourceReplacements/RoactCompat.lua"),
};
