(function() {
    var implementors = Object.fromEntries([["nalgebra",[["impl&lt;T, R, C&gt; <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a> for <a class=\"type\" href=\"nalgebra/base/type.OMatrix.html\" title=\"type nalgebra::base::OMatrix\">OMatrix</a>&lt;T, R, C&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a> + <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a>,\n    R: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>,\n    C: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>,\n    T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>,\n    <a class=\"struct\" href=\"nalgebra/base/default_allocator/struct.DefaultAllocator.html\" title=\"struct nalgebra::base::default_allocator::DefaultAllocator\">DefaultAllocator</a>: <a class=\"trait\" href=\"nalgebra/base/allocator/trait.Allocator.html\" title=\"trait nalgebra::base::allocator::Allocator\">Allocator</a>&lt;R, C&gt;,</div>"],["impl&lt;T, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a> for <a class=\"struct\" href=\"nalgebra/geometry/struct.Rotation.html\" title=\"struct nalgebra::geometry::Rotation\">Rotation</a>&lt;T, D&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a> + <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a>,\n    T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a> + <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a>&gt; <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a> for <a class=\"struct\" href=\"nalgebra/geometry/struct.Quaternion.html\" title=\"struct nalgebra::geometry::Quaternion\">Quaternion</a>&lt;T&gt;<div class=\"where\">where\n    T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a> + <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a>&gt; <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a> for <a class=\"type\" href=\"nalgebra/geometry/type.UnitQuaternion.html\" title=\"type nalgebra::geometry::UnitQuaternion\">UnitQuaternion</a>&lt;T&gt;<div class=\"where\">where\n    T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a> + <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a>, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a> for <a class=\"struct\" href=\"nalgebra/geometry/struct.Scale.html\" title=\"struct nalgebra::geometry::Scale\">Scale</a>&lt;T, D&gt;<div class=\"where\">where\n    T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a> + <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a>, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a> for <a class=\"struct\" href=\"nalgebra/geometry/struct.Translation.html\" title=\"struct nalgebra::geometry::Translation\">Translation</a>&lt;T, D&gt;<div class=\"where\">where\n    T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a> + <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a>, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a> for <a class=\"type\" href=\"nalgebra/geometry/type.Point.html\" title=\"type nalgebra::geometry::Point\">Point</a>&lt;T, D&gt;<div class=\"where\">where\n    T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/trait.RealField.html\" title=\"trait nalgebra::RealField\">RealField</a>, C, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a> for <a class=\"struct\" href=\"nalgebra/geometry/struct.Transform.html\" title=\"struct nalgebra::geometry::Transform\">Transform</a>&lt;T, C, D&gt;<div class=\"where\">where\n    T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>,\n    C: <a class=\"trait\" href=\"nalgebra/geometry/trait.TCategory.html\" title=\"trait nalgebra::geometry::TCategory\">TCategory</a>,\n    <a class=\"struct\" href=\"nalgebra/base/dimension/struct.Const.html\" title=\"struct nalgebra::base::dimension::Const\">Const</a>&lt;D&gt;: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimNameAdd.html\" title=\"trait nalgebra::base::dimension::DimNameAdd\">DimNameAdd</a>&lt;<a class=\"type\" href=\"nalgebra/base/dimension/type.U1.html\" title=\"type nalgebra::base::dimension::U1\">U1</a>&gt;,\n    <a class=\"struct\" href=\"nalgebra/base/default_allocator/struct.DefaultAllocator.html\" title=\"struct nalgebra::base::default_allocator::DefaultAllocator\">DefaultAllocator</a>: <a class=\"trait\" href=\"nalgebra/base/allocator/trait.Allocator.html\" title=\"trait nalgebra::base::allocator::Allocator\">Allocator</a>&lt;<a class=\"type\" href=\"nalgebra/base/dimension/type.DimNameSum.html\" title=\"type nalgebra::base::dimension::DimNameSum\">DimNameSum</a>&lt;<a class=\"struct\" href=\"nalgebra/base/dimension/struct.Const.html\" title=\"struct nalgebra::base::dimension::Const\">Const</a>&lt;D&gt;, <a class=\"type\" href=\"nalgebra/base/dimension/type.U1.html\" title=\"type nalgebra::base::dimension::U1\">U1</a>&gt;, <a class=\"type\" href=\"nalgebra/base/dimension/type.DimNameSum.html\" title=\"type nalgebra::base::dimension::DimNameSum\">DimNameSum</a>&lt;<a class=\"struct\" href=\"nalgebra/base/dimension/struct.Const.html\" title=\"struct nalgebra::base::dimension::Const\">Const</a>&lt;D&gt;, <a class=\"type\" href=\"nalgebra/base/dimension/type.U1.html\" title=\"type nalgebra::base::dimension::U1\">U1</a>&gt;&gt;,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>&gt; <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a> for <a class=\"type\" href=\"nalgebra/geometry/type.UnitComplex.html\" title=\"type nalgebra::geometry::UnitComplex\">UnitComplex</a>&lt;T&gt;<div class=\"where\">where\n    T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>, R, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a> for <a class=\"struct\" href=\"nalgebra/geometry/struct.Isometry.html\" title=\"struct nalgebra::geometry::Isometry\">Isometry</a>&lt;T, R, D&gt;<div class=\"where\">where\n    T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>,\n    R: <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a>&lt;SimdBool = T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.SimdBool\" title=\"type nalgebra::SimdValue::SimdBool\">SimdBool</a>&gt; + <a class=\"trait\" href=\"nalgebra/geometry/trait.AbstractRotation.html\" title=\"trait nalgebra::geometry::AbstractRotation\">AbstractRotation</a>&lt;T, D&gt;,\n    R::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/geometry/trait.AbstractRotation.html\" title=\"trait nalgebra::geometry::AbstractRotation\">AbstractRotation</a>&lt;T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>, D&gt;,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>, R, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a> for <a class=\"struct\" href=\"nalgebra/geometry/struct.Similarity.html\" title=\"struct nalgebra::geometry::Similarity\">Similarity</a>&lt;T, R, D&gt;<div class=\"where\">where\n    T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>,\n    R: <a class=\"trait\" href=\"nalgebra/trait.SimdValue.html\" title=\"trait nalgebra::SimdValue\">SimdValue</a>&lt;SimdBool = T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.SimdBool\" title=\"type nalgebra::SimdValue::SimdBool\">SimdBool</a>&gt; + <a class=\"trait\" href=\"nalgebra/geometry/trait.AbstractRotation.html\" title=\"trait nalgebra::geometry::AbstractRotation\">AbstractRotation</a>&lt;T, D&gt;,\n    R::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/geometry/trait.AbstractRotation.html\" title=\"trait nalgebra::geometry::AbstractRotation\">AbstractRotation</a>&lt;T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>, D&gt;,</div>"]]],["simba",[]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[13219,13]}