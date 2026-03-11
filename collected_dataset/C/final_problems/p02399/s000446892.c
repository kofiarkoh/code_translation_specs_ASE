#include<stdio.h>

int func1(int a,int b){
	return a/b;
}
int func2(int a,int b){
	return a%b;
}

double func3(int a,int b){
	return (double)a/b;
}
int main(){
	int A,B;
	int i;
	
	scanf("%d%d",&A,&B);
	
	
	printf("%d %d %0.5lf\n",func1(A,B),func2(A,B),func3(A,B));
	
	return 0;
}