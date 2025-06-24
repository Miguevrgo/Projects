define i32 @main() {
entry:
	%t0 = add i32 3, 3
	%t1 = mul i32 4, %t0
	%t2 = sdiv i32 %t1, 3
	%t3 = mul i32 %t2, 2
	%t4 = add i32 %t3, 3
	%t5 = sub i32 3, 5
	%t6 = mul i32 1, %t5
	%t7 = sub i32 %t4, %t6
  ret i32 %t7
}
