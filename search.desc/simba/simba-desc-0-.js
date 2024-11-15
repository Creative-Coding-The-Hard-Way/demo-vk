searchState.loadedDescShard("simba", 0, "<strong>Simba</strong> is a crate defining a set of trait for writing code …\nTraits implemented by scalar, non-SIMD, types.\nTraits implemented by SIMD types and non-SIMD types.\nTrait <strong>alias</strong> for <code>Add</code> with result of type <code>Self</code>.\nTrait <strong>alias</strong> for <code>Add</code> and <code>AddAssign</code> with result of type <code>Self</code>.\nTrait <strong>alias</strong> for <code>Div</code> with result of type <code>Self</code>.\nTrait <strong>alias</strong> for <code>Div</code> and <code>DivAssign</code> with result of type <code>Self</code>.\nTrait <strong>alias</strong> for <code>Mul</code> with result of type <code>Self</code>.\nTrait <strong>alias</strong> for <code>Mul</code> and <code>MulAssign</code> with result of type <code>Self</code>.\nTrait <strong>alias</strong> for <code>Neg</code> with result of type <code>Self</code>.\nTrait <strong>alias</strong> for <code>Sub</code> with result of type <code>Self</code>.\nTrait <strong>alias</strong> for <code>Sub</code> and <code>SubAssign</code> with result of type <code>Self</code>.\nTrait shared by all complex fields and its subfields (like …\nTrait implemented by fields, i.e., complex numbers and …\nTrait shared by all reals.\nNested sets and conversions between them (using an …\nNested sets and conversions between them. Useful to work …\nThe absolute value of this complex number: …\nThe argument of this complex number.\nCopies the sign of <code>sign</code> to <code>self</code>.\nCardinal cos\nCardinal cos\nBuilds a pure-real complex number from the given value.\nThe inclusion map: converts <code>self</code> to the equivalent element …\nThe inverse inclusion map: attempts to construct <code>self</code> from …\nThe inverse inclusion map: attempts to construct <code>self</code> from …\nUse with care! Same as <code>self.to_superset</code> but without any …\nComputes (self.conjugate() * self + other.conjugate() * …\nThe imaginary part of this complex number.\nChecks if <code>element</code> is actually part of the subset <code>Self</code> (and …\nChecks if <code>self</code> is actually part of its subset <code>T</code> (and can …\nIs the sign of this real number negative?\nIs the sign of this real number positive?\nThe largest finite positive value representable using this …\nThe smallest finite positive value representable using …\nThe modulus of this complex number.\nThe squared modulus of this complex number.\nThe sum of the absolute value of this complex number’s …\nThe real part of this complex number.\nMultiplies this complex number by <code>factor</code>.\nThe exponential part of this complex number: …\nThe exponential part of this complex number: …\nCardinal sine\nCardinal sine\nThe exponential form of this complex number: (modulus, …\nThe exponential form of this complex number: (modulus, …\nThe polar form of this complex number: (modulus, arg)\nThe polar form of this complex number: (modulus, arg)\nThe inverse inclusion map: attempts to construct <code>self</code> from …\nThe inverse inclusion map: attempts to construct <code>self</code> from …\nUse with care! Same as <code>self.to_subset</code> but without any …\nThe inclusion map: converts <code>self</code> to the equivalent element …\nDivides this complex number by <code>factor</code>.\nA SIMD boolean structure that implements all the relevant …\nA SIMD structure that implements all the relevant traits …\nThe type of the elements of each lane of this SIMD value.\nThe number of lanes of this SIMD value.\nMarker trait implemented by SIMD and non-SIMD primitive …\nLane-wise generalization of <code>bool</code> for SIMD booleans.\nType of the result of comparing two SIMD values like <code>self</code>.\nLane-wise generalisation of <code>ComplexField</code> for SIMD complex …\nGeneralization of Option for SIMD computation.\nLane-wise generalization of the standard <code>PartialOrd</code> for …\nLanewise generalization of <code>RealField</code> for SIMD reals.\nType of the coefficients of a complex number.\nA lane-wise generalization of <code>num::Signed</code> for SIMD values.\nBase trait for every SIMD types.\nAn SIMD boolean structure associated to <code>wide::f32x4</code> that …\nAn SIMD boolean structure associated to <code>wide::f32x8</code> that …\nAn SIMD boolean structure associated to <code>wide::f64x4</code> that …\nA wrapper type of <code>wide::f32x4</code> that implements all the …\nA wrapper type of <code>wide::f32x8</code> that implements all the …\nA wrapper type of <code>wide::f64x4</code> that implements all the …\nAre all vector lanes true?\nLane-wise bitwise and of the vector elements.\nIs any vector lane true?\nA bit mask representing the boolean state of each lanes of …\nExtracts the i-th lane of <code>self</code>.\nExtracts the i-th lane of <code>self</code> without bound-checking.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nBuilds a pure-real complex number from the given value.\nMerges the value of <code>if_value()</code> and <code>else_value()</code> depending …\nMerges the value of <code>if_value()</code> and <code>else_if.1()</code> and …\nMerges the value of <code>if_value()</code> and <code>else_if.1()</code> and …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nTests which lane is negative.\nTests which lane is positive.\nApplies a function to each lane of <code>self</code>.\nApplies a function to each lane of <code>self</code>.\nReturn the underlying SIMD boolean mask.\nCreates a new SIMD option by combining a value and a mask …\nAre all vector lanes false?\nConverts this SIMD option to a strandard Option.\nLane-wise bitwise or of the vector elements.\nReplaces the i-th lane of <code>self</code> by <code>val</code>.\nReplaces the i-th lane of <code>self</code> by <code>val</code> without …\nMerges <code>self</code> and <code>other</code> depending on the lanes of <code>cond</code>.\nThe absolute value of this complex number: …\nThe absolute value of each lane of <code>self</code>.\nThe absolute difference of each lane of <code>self</code>.\nThe argument of this complex number.\nClamps each lane of <code>self</code> between the corresponding lane of …\nCopies the sign of <code>sign</code> to <code>self</code>.\nCardinal cos\nCardinal cos\nLanewise <em>equal</em> <code>==</code> comparison.\nLanewise <em>greater or equal</em> <code>&gt;=</code> comparison.\nLanewise <em>greater than</em> <code>&gt;</code> comparison.\nThe max value among all lanes of <code>self</code>.\nThe min value among all lanes of <code>self</code>.\nComputes the product of all the lanes of <code>self</code>.\nComputes the sum of all the lanes of <code>self</code>.\nComputes (self.conjugate() * self + other.conjugate() * …\nThe imaginary part of this complex number.\nLanewise <em>less or equal</em> <code>&lt;=</code> comparison.\nLanewise <em>less than</em> <code>&lt;</code> comparison.\nLanewise max value.\nLanewise min value.\nThe modulus of this complex number.\nThe squared modulus of this complex number.\nLanewise <em>not equal</em> <code>!=</code> comparison.\nThe sum of the absolute value of this complex number’s …\nThe real part of this complex number.\nMultiplies this complex number by <code>factor</code>.\nThe signum of each lane of <code>Self</code>.\nThe exponential part of this complex number: …\nThe exponential part of this complex number: …\nCardinal sine\nCardinal sine\nThe exponential form of this complex number: (modulus, …\nThe exponential form of this complex number: (modulus, …\nThe polar form of this complex number: (modulus, arg)\nThe polar form of this complex number: (modulus, arg)\nDivides this complex number by <code>factor</code>.\nRetrieve the underlying value if all the bits of <code>self.mask</code> …\nMerges the value of <code>self</code> with the value of <code>other</code>.\nInitializes an SIMD value with each lanes set to <code>val</code>.\nReturn the underlying unfiltered value.\nLane-wise bitwise xor of the vector elements.\nApplies a function to each lane of <code>self</code> paired with the …\nApplies a function to each lane of <code>self</code> paired with the …")