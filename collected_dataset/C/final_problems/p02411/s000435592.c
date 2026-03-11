#include<stdio.h>
char check(int test1 , int test2 , int test3) {
	int sumScore = test1 + test2;
	if (test1 == -1 || test2 == -1) {
		return 'F';
	}
	if (sumScore >= 80) {
		return 'A';
	}
	if (sumScore >= 65) {
		return 'B';
	}
	if (sumScore >= 50) {
		return 'C';
	}
	if (sumScore >= 30) {
		if (test3 >= 50) {
			return 'C';
		} else {
			return 'D';
		}	
	}
	return 'F';
}
int main () {
	int test1 , test2 , test3;
	while(1) {
		scanf("%d %d %d" , &test1 , &test2 , &test3);
		if (test1 == -1 && test2 == -1 && test3 == -1) break;
		printf("%c\n" , check(test1 , test2 , test3));
	}
	return 0;
}