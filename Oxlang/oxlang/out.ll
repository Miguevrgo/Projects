define i32 @main() {
entry:
	%t0 = mul i32 3, 4
	%t1 = sdiv i32 %t0, 2
	%t2 = sub i32 %t1, 5
	ret i32 %t2
}
