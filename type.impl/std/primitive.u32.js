(function() {
    var type_impls = Object.fromEntries([["ash",[]],["nix",[]],["spin_sleep",[]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[10,11,18]}