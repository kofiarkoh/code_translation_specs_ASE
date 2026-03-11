#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<math.h>
#define PI 3.141592653589793

int main(){
	int a, b, c, d;
	scanf("%d%d%d%d", &a, &b, &c, &d);
	int answer;
	if(a*b>c*d){
		answer = a*b;
	}
	else answer = c*d;
	printf("%d\n", answer);

	return 0;
}