#include <stdio.h>
#include <math.h>
#include <string.h>

int Mymax(int a,int b);
int Mymin(int a,int b);
int main(){
	int N;
	int A,B;
	int i;
	scanf("%d",&N);
	scanf("%d %d",&A,&B);
	int P[N+1];
	for(i=1;i<=N;i++){
		scanf("%d",&P[i]);
	//	printf("%d\n",P[i]);
	}
	
	int problem1[N+1],problem2[N+1],problem3[N+1];
	int index1=0,index2=0,index3=0;
	//与えられた問題の分類
	for(i=1;i<=N;i++){
		if(P[i] <= A){
			index1 += 1;
			problem1[index1] = i;
		}
		if((A < P[i])&&(P[i] <= B)){
			index2 += 1;
			problem2[index2] = i;
		}
		if(B < P[i]){
			index3 += 1;
			problem3[index3] = i;
		}
	}
	
	int x;
	x = Mymin(index1,Mymin(index2,index3));
	printf("%d\n",x);
	
	return 0;
}

int Mymax(int a,int b){
	int r;
	if(a > b){
		r = a;
	}else{
		r = b;
	}
	return r;
}

int Mymin(int a,int b){
	int r;
	if(a < b){
		r = a;
	}else{
		r = b;
	}
	return r;
}