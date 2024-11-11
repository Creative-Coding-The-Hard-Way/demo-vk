(function() {
    var type_impls = Object.fromEntries([["nalgebra",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3C%26mut+Matrix%3CT,+R,+C,+S%3E%3E-for-Matrix%3CT,+RView,+CView,+ViewStorageMut%3C'a,+T,+RView,+CView,+RStride,+CStride%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/nalgebra/base/conversion.rs.html#464-496\">source</a><a href=\"#impl-From%3C%26mut+Matrix%3CT,+R,+C,+S%3E%3E-for-Matrix%3CT,+RView,+CView,+ViewStorageMut%3C'a,+T,+RView,+CView,+RStride,+CStride%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T, R, C, RView, CView, RStride, CStride, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a mut <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, R, C, S&gt;&gt; for <a class=\"type\" href=\"nalgebra/base/type.MatrixViewMut.html\" title=\"type nalgebra::base::MatrixViewMut\">MatrixViewMut</a>&lt;'a, T, RView, CView, RStride, CStride&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>,\n    C: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>,\n    RView: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>,\n    CView: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>,\n    RStride: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>,\n    CStride: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>,\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorageMut.html\" title=\"trait nalgebra::base::storage::RawStorageMut\">RawStorageMut</a>&lt;T, R, C&gt;,\n    <a class=\"struct\" href=\"nalgebra/base/constraint/struct.ShapeConstraint.html\" title=\"struct nalgebra::base::constraint::ShapeConstraint\">ShapeConstraint</a>: <a class=\"trait\" href=\"nalgebra/base/constraint/trait.DimEq.html\" title=\"trait nalgebra::base::constraint::DimEq\">DimEq</a>&lt;R, RView&gt; + <a class=\"trait\" href=\"nalgebra/base/constraint/trait.DimEq.html\" title=\"trait nalgebra::base::constraint::DimEq\">DimEq</a>&lt;C, CView&gt; + <a class=\"trait\" href=\"nalgebra/base/constraint/trait.DimEq.html\" title=\"trait nalgebra::base::constraint::DimEq\">DimEq</a>&lt;RStride, S::<a class=\"associatedtype\" href=\"nalgebra/base/storage/trait.RawStorage.html#associatedtype.RStride\" title=\"type nalgebra::base::storage::RawStorage::RStride\">RStride</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/constraint/trait.DimEq.html\" title=\"trait nalgebra::base::constraint::DimEq\">DimEq</a>&lt;CStride, S::<a class=\"associatedtype\" href=\"nalgebra/base/storage/trait.RawStorage.html#associatedtype.CStride\" title=\"type nalgebra::base::storage::RawStorage::CStride\">CStride</a>&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/nalgebra/base/conversion.rs.html#477-495\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(m: &amp;'a mut <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, R, C, S&gt;) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<&'a mut Matrix<T, R, C, S>>","nalgebra::base::matrix_view::MatrixSliceMut"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Matrix%3CT,+Dyn,+C,+ViewStorageMut%3C'a,+T,+Dyn,+C,+Const%3C1%3E,+Dyn%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#310-313\">source</a><a href=\"#impl-Matrix%3CT,+Dyn,+C,+ViewStorageMut%3C'a,+T,+Dyn,+C,+Const%3C1%3E,+Dyn%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, C: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>&gt; <a class=\"type\" href=\"nalgebra/base/type.MatrixViewMut.html\" title=\"type nalgebra::base::MatrixViewMut\">MatrixViewMut</a>&lt;'a, T, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.Dyn.html\" title=\"struct nalgebra::base::dimension::Dyn\">Dyn</a>, C&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#310-313\">source</a><h4 class=\"code-header\">pub fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice\" class=\"fn\">from_slice</a>(data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>, nrows: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates a new mutable matrix view from the given data array.</p>\n<p>Panics if <code>data</code> does not contain enough elements.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice_unchecked\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#310-313\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice_unchecked\" class=\"fn\">from_slice_unchecked</a>(\n    data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>,\n    start: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    nrows: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates, without bound checking, a new mutable matrix view from the given data array.</p>\n<h5 id=\"safety\"><a class=\"doc-anchor\" href=\"#safety\">§</a>Safety</h5>\n<p><code>data[start..start+(R * C)]</code> must be within bounds.</p>\n</div></details></div></details>",0,"nalgebra::base::matrix_view::MatrixSliceMut"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Matrix%3CT,+Dyn,+C,+ViewStorageMut%3C'a,+T,+Dyn,+C,+Dyn,+Dyn%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#310-313\">source</a><a href=\"#impl-Matrix%3CT,+Dyn,+C,+ViewStorageMut%3C'a,+T,+Dyn,+C,+Dyn,+Dyn%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, C: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>&gt; <a class=\"type\" href=\"nalgebra/base/type.MatrixViewMut.html\" title=\"type nalgebra::base::MatrixViewMut\">MatrixViewMut</a>&lt;'a, T, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.Dyn.html\" title=\"struct nalgebra::base::dimension::Dyn\">Dyn</a>, C, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.Dyn.html\" title=\"struct nalgebra::base::dimension::Dyn\">Dyn</a>, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.Dyn.html\" title=\"struct nalgebra::base::dimension::Dyn\">Dyn</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice_with_strides_mut\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#310-313\">source</a><h4 class=\"code-header\">pub fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice_with_strides_mut\" class=\"fn\">from_slice_with_strides_mut</a>(\n    data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>,\n    nrows: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    rstride: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    cstride: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates a new mutable matrix view with the specified strides from the given data array.</p>\n<p>Panics if <code>data</code> does not contain enough elements.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice_with_strides_unchecked\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#310-313\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice_with_strides_unchecked\" class=\"fn\">from_slice_with_strides_unchecked</a>(\n    data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>,\n    start: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    nrows: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    rstride: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    cstride: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates, without bound checking, a new mutable matrix view with the specified strides from the given data array.</p>\n<h5 id=\"safety\"><a class=\"doc-anchor\" href=\"#safety\">§</a>Safety</h5>\n<p><code>data[start..start+rstride * cstride]</code> must be within bounds.</p>\n</div></details></div></details>",0,"nalgebra::base::matrix_view::MatrixSliceMut"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Matrix%3CT,+Dyn,+Dyn,+ViewStorageMut%3C'a,+T,+Dyn,+Dyn,+Const%3C1%3E,+Dyn%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#315-318\">source</a><a href=\"#impl-Matrix%3CT,+Dyn,+Dyn,+ViewStorageMut%3C'a,+T,+Dyn,+Dyn,+Const%3C1%3E,+Dyn%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>&gt; <a class=\"type\" href=\"nalgebra/base/type.MatrixViewMut.html\" title=\"type nalgebra::base::MatrixViewMut\">MatrixViewMut</a>&lt;'a, T, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.Dyn.html\" title=\"struct nalgebra::base::dimension::Dyn\">Dyn</a>, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.Dyn.html\" title=\"struct nalgebra::base::dimension::Dyn\">Dyn</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#315-318\">source</a><h4 class=\"code-header\">pub fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice\" class=\"fn\">from_slice</a>(data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>, nrows: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>, ncols: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates a new mutable matrix view from the given data array.</p>\n<p>Panics if <code>data</code> does not contain enough elements.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice_unchecked\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#315-318\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice_unchecked\" class=\"fn\">from_slice_unchecked</a>(\n    data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>,\n    start: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    nrows: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    ncols: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates, without bound checking, a new mutable matrix view from the given data array.</p>\n<h5 id=\"safety\"><a class=\"doc-anchor\" href=\"#safety\">§</a>Safety</h5>\n<p><code>data[start..start+(R * C)]</code> must be within bounds.</p>\n</div></details></div></details>",0,"nalgebra::base::matrix_view::MatrixSliceMut"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Matrix%3CT,+Dyn,+Dyn,+ViewStorageMut%3C'a,+T,+Dyn,+Dyn,+Dyn,+Dyn%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#315-318\">source</a><a href=\"#impl-Matrix%3CT,+Dyn,+Dyn,+ViewStorageMut%3C'a,+T,+Dyn,+Dyn,+Dyn,+Dyn%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>&gt; <a class=\"type\" href=\"nalgebra/base/type.MatrixViewMut.html\" title=\"type nalgebra::base::MatrixViewMut\">MatrixViewMut</a>&lt;'a, T, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.Dyn.html\" title=\"struct nalgebra::base::dimension::Dyn\">Dyn</a>, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.Dyn.html\" title=\"struct nalgebra::base::dimension::Dyn\">Dyn</a>, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.Dyn.html\" title=\"struct nalgebra::base::dimension::Dyn\">Dyn</a>, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.Dyn.html\" title=\"struct nalgebra::base::dimension::Dyn\">Dyn</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice_with_strides_mut\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#315-318\">source</a><h4 class=\"code-header\">pub fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice_with_strides_mut\" class=\"fn\">from_slice_with_strides_mut</a>(\n    data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>,\n    nrows: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    ncols: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    rstride: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    cstride: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates a new mutable matrix view with the specified strides from the given data array.</p>\n<p>Panics if <code>data</code> does not contain enough elements.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice_with_strides_unchecked\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#315-318\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice_with_strides_unchecked\" class=\"fn\">from_slice_with_strides_unchecked</a>(\n    data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>,\n    start: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    nrows: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    ncols: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    rstride: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    cstride: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates, without bound checking, a new mutable matrix view with the specified strides from the given data array.</p>\n<h5 id=\"safety\"><a class=\"doc-anchor\" href=\"#safety\">§</a>Safety</h5>\n<p><code>data[start..start+rstride * cstride]</code> must be within bounds.</p>\n</div></details></div></details>",0,"nalgebra::base::matrix_view::MatrixSliceMut"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Matrix%3CT,+R,+C,+ViewStorageMut%3C'a,+T,+R,+C,+Const%3C1%3E,+R%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#227-253\">source</a><a href=\"#impl-Matrix%3CT,+R,+C,+ViewStorageMut%3C'a,+T,+R,+C,+Const%3C1%3E,+R%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, R: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, C: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>&gt; <a class=\"type\" href=\"nalgebra/base/type.MatrixViewMut.html\" title=\"type nalgebra::base::MatrixViewMut\">MatrixViewMut</a>&lt;'a, T, R, C&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice_generic_unchecked\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#234-243\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice_generic_unchecked\" class=\"fn\">from_slice_generic_unchecked</a>(\n    data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>,\n    start: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    nrows: R,\n    ncols: C,\n) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates, without bound-checking, a mutable matrix view from an array and with dimensions specified by generic types instances.</p>\n<h5 id=\"safety\"><a class=\"doc-anchor\" href=\"#safety\">§</a>Safety</h5>\n<p>This method is unsafe because the input data array is not checked to contain enough elements.\nThe generic types <code>R</code> and <code>C</code> can either be type-level integers or integers wrapped with <code>Dyn()</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice_generic\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#250-252\">source</a><h4 class=\"code-header\">pub fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice_generic\" class=\"fn\">from_slice_generic</a>(data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>, nrows: R, ncols: C) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates a mutable matrix view from an array and with dimensions and strides specified by generic types instances.</p>\n<p>Panics if the input data array dose not contain enough elements.\nThe generic types <code>R</code> and <code>C</code> can either be type-level integers or integers wrapped with <code>Dyn()</code>.</p>\n</div></details></div></details>",0,"nalgebra::base::matrix_view::MatrixSliceMut"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Matrix%3CT,+R,+C,+ViewStorageMut%3C'a,+T,+R,+C,+Const%3C1%3E,+R%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#300-303\">source</a><a href=\"#impl-Matrix%3CT,+R,+C,+ViewStorageMut%3C'a,+T,+R,+C,+Const%3C1%3E,+R%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, R: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>, C: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>&gt; <a class=\"type\" href=\"nalgebra/base/type.MatrixViewMut.html\" title=\"type nalgebra::base::MatrixViewMut\">MatrixViewMut</a>&lt;'a, T, R, C&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#300-303\">source</a><h4 class=\"code-header\">pub fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice\" class=\"fn\">from_slice</a>(data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates a new mutable matrix view from the given data array.</p>\n<p>Panics if <code>data</code> does not contain enough elements.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice_unchecked\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#300-303\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice_unchecked\" class=\"fn\">from_slice_unchecked</a>(data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>, start: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates, without bound checking, a new mutable matrix view from the given data array.</p>\n<h5 id=\"safety\"><a class=\"doc-anchor\" href=\"#safety\">§</a>Safety</h5>\n<p><code>data[start..start+(R * C)]</code> must be within bounds.</p>\n</div></details></div></details>",0,"nalgebra::base::matrix_view::MatrixSliceMut"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Matrix%3CT,+R,+C,+ViewStorageMut%3C'a,+T,+R,+C,+Dyn,+Dyn%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#300-303\">source</a><a href=\"#impl-Matrix%3CT,+R,+C,+ViewStorageMut%3C'a,+T,+R,+C,+Dyn,+Dyn%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, R: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>, C: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>&gt; <a class=\"type\" href=\"nalgebra/base/type.MatrixViewMut.html\" title=\"type nalgebra::base::MatrixViewMut\">MatrixViewMut</a>&lt;'a, T, R, C, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.Dyn.html\" title=\"struct nalgebra::base::dimension::Dyn\">Dyn</a>, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.Dyn.html\" title=\"struct nalgebra::base::dimension::Dyn\">Dyn</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice_with_strides_mut\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#300-303\">source</a><h4 class=\"code-header\">pub fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice_with_strides_mut\" class=\"fn\">from_slice_with_strides_mut</a>(\n    data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>,\n    rstride: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    cstride: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates a new mutable matrix view with the specified strides from the given data array.</p>\n<p>Panics if <code>data</code> does not contain enough elements.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice_with_strides_unchecked\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#300-303\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice_with_strides_unchecked\" class=\"fn\">from_slice_with_strides_unchecked</a>(\n    data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>,\n    start: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    rstride: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    cstride: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates, without bound checking, a new mutable matrix view with the specified strides from the given data array.</p>\n<h5 id=\"safety\"><a class=\"doc-anchor\" href=\"#safety\">§</a>Safety</h5>\n<p><code>data[start..start+rstride * cstride]</code> must be within bounds.</p>\n</div></details></div></details>",0,"nalgebra::base::matrix_view::MatrixSliceMut"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Matrix%3CT,+R,+C,+ViewStorageMut%3C'a,+T,+R,+C,+RStride,+CStride%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#153-225\">source</a><a href=\"#impl-Matrix%3CT,+R,+C,+ViewStorageMut%3C'a,+T,+R,+C,+RStride,+CStride%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, R: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, C: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, RStride: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, CStride: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>&gt; <a class=\"type\" href=\"nalgebra/base/type.MatrixViewMut.html\" title=\"type nalgebra::base::MatrixViewMut\">MatrixViewMut</a>&lt;'a, T, R, C, RStride, CStride&gt;</h3></section></summary><div class=\"docblock\"><h4 id=\"creating-mutable-matrix-views-from-mut-t\"><a class=\"doc-anchor\" href=\"#creating-mutable-matrix-views-from-mut-t\">§</a>Creating mutable matrix views from <code>&amp;mut [T]</code></h4></div><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice_with_strides_generic_unchecked\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#162-176\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice_with_strides_generic_unchecked\" class=\"fn\">from_slice_with_strides_generic_unchecked</a>(\n    data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>,\n    start: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    nrows: R,\n    ncols: C,\n    rstride: RStride,\n    cstride: CStride,\n) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates, without bound-checking, a mutable matrix view from an array and with dimensions and strides specified by generic types instances.</p>\n<h5 id=\"safety\"><a class=\"doc-anchor\" href=\"#safety\">§</a>Safety</h5>\n<p>This method is unsafe because the input data array is not checked to contain enough elements.\nThe generic types <code>R</code>, <code>C</code>, <code>RStride</code>, <code>CStride</code> can either be type-level integers or integers wrapped with <code>Dyn()</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice_with_strides_generic\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#183-224\">source</a><h4 class=\"code-header\">pub fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice_with_strides_generic\" class=\"fn\">from_slice_with_strides_generic</a>(\n    data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>,\n    nrows: R,\n    ncols: C,\n    rstride: RStride,\n    cstride: CStride,\n) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates a mutable matrix view from an array and with dimensions and strides specified by generic types instances.</p>\n<p>Panics if the input data array dose not contain enough elements.\nThe generic types <code>R</code>, <code>C</code>, <code>RStride</code>, <code>CStride</code> can either be type-level integers or integers wrapped with <code>Dyn()</code>.</p>\n</div></details></div></details>",0,"nalgebra::base::matrix_view::MatrixSliceMut"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Matrix%3CT,+R,+Dyn,+ViewStorageMut%3C'a,+T,+R,+Dyn,+Const%3C1%3E,+R%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#305-308\">source</a><a href=\"#impl-Matrix%3CT,+R,+Dyn,+ViewStorageMut%3C'a,+T,+R,+Dyn,+Const%3C1%3E,+R%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, R: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>&gt; <a class=\"type\" href=\"nalgebra/base/type.MatrixViewMut.html\" title=\"type nalgebra::base::MatrixViewMut\">MatrixViewMut</a>&lt;'a, T, R, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.Dyn.html\" title=\"struct nalgebra::base::dimension::Dyn\">Dyn</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#305-308\">source</a><h4 class=\"code-header\">pub fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice\" class=\"fn\">from_slice</a>(data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>, ncols: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates a new mutable matrix view from the given data array.</p>\n<p>Panics if <code>data</code> does not contain enough elements.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice_unchecked\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#305-308\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice_unchecked\" class=\"fn\">from_slice_unchecked</a>(\n    data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>,\n    start: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    ncols: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates, without bound checking, a new mutable matrix view from the given data array.</p>\n<h5 id=\"safety\"><a class=\"doc-anchor\" href=\"#safety\">§</a>Safety</h5>\n<p><code>data[start..start+(R * C)]</code> must be within bounds.</p>\n</div></details></div></details>",0,"nalgebra::base::matrix_view::MatrixSliceMut"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Matrix%3CT,+R,+Dyn,+ViewStorageMut%3C'a,+T,+R,+Dyn,+Dyn,+Dyn%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#305-308\">source</a><a href=\"#impl-Matrix%3CT,+R,+Dyn,+ViewStorageMut%3C'a,+T,+R,+Dyn,+Dyn,+Dyn%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, R: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>&gt; <a class=\"type\" href=\"nalgebra/base/type.MatrixViewMut.html\" title=\"type nalgebra::base::MatrixViewMut\">MatrixViewMut</a>&lt;'a, T, R, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.Dyn.html\" title=\"struct nalgebra::base::dimension::Dyn\">Dyn</a>, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.Dyn.html\" title=\"struct nalgebra::base::dimension::Dyn\">Dyn</a>, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.Dyn.html\" title=\"struct nalgebra::base::dimension::Dyn\">Dyn</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice_with_strides_mut\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#305-308\">source</a><h4 class=\"code-header\">pub fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice_with_strides_mut\" class=\"fn\">from_slice_with_strides_mut</a>(\n    data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>,\n    ncols: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    rstride: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    cstride: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates a new mutable matrix view with the specified strides from the given data array.</p>\n<p>Panics if <code>data</code> does not contain enough elements.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_slice_with_strides_unchecked\" class=\"method\"><a class=\"src rightside\" href=\"src/nalgebra/base/construction_view.rs.html#305-308\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"nalgebra/base/type.MatrixViewMut.html#tymethod.from_slice_with_strides_unchecked\" class=\"fn\">from_slice_with_strides_unchecked</a>(\n    data: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.slice.html\">[T]</a>,\n    start: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    ncols: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    rstride: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n    cstride: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>,\n) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates, without bound checking, a new mutable matrix view with the specified strides from the given data array.</p>\n<h5 id=\"safety\"><a class=\"doc-anchor\" href=\"#safety\">§</a>Safety</h5>\n<p><code>data[start..start+rstride * cstride]</code> must be within bounds.</p>\n</div></details></div></details>",0,"nalgebra::base::matrix_view::MatrixSliceMut"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[37528]}