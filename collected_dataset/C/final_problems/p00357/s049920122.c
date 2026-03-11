#include <stdio.h>
int d[300005];

int check1(int s);
int check2(int s,int N);

int main(void) {
	int N,i,f;
	scanf("%d",&N);
	for(i=0;i<N;i++){
		scanf("%d",&d[i]);
	}
	f=check1(N-1);
	if(f==0){printf("no\n");return 0;}
	f=check2(0,N-1);
	if(f==0){printf("no\n");return 0;}
	else {printf("yes\n");}
	return 0;
}

int check1(int s){
	int i,f;
	if(s==0){return 1;}
	for(i=s-1;i>=0;i--){
		if(d[i]>=10*(s-i)){
			f=check1(i);
			if(f==0){return 0;}else{return 1;}
		}
	}
	return 0;
}

int check2(int s,int N){
	int i,f;
	if(s==N){return 1;}
	for(i=s+1;i<=N;i++){
		if(d[i]>=10*(i-s)){
			f=check2(i,N);
			if(f==0){return 0;}else{return 1;}
		}
	}
	return 0;
}

