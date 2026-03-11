#include <limits.h>
#include <stdio.h>
#include <string.h>

int A[200001], a=0;
int B[200001], b=0;

int n, m, k, i, j;
char S[200001];

void printS(void){
	int i=200000;
	while((S[i]&1)==0) i--;
	for(; i>=0; i--){
		putchar((S[i]&1)==0?'0':'1');
	}putchar(10);
}

void printT(void){
	int i=200000;
	while((S[i]&2)==0) i--;
	for(; i>=0; i--){
		putchar((S[i]&2)==0?'0':'1');
	}putchar(10);
}

int main(void){
	scanf("%d%d%d\n", &n, &m, &k);
	for(i=n-1; i>=0; i--){
		S[i]=getchar()-'0';
	}
	while(getchar()!=10);
	for(i=m-1; i>=0; i--){
		if(getchar()=='1') S[i]|=2;
	}
	int l = n>m?n:m;
	for(i=l; i>=0; i--){
		if(S[i]==3){
			S[i]=0;
			int z=k; j=i;
			char x=3;
			while(1){
				int p=a?A[--a]:INT_MAX;
				if(p>j+1){
					if(x==3){
						if(p>j+z){
							S[j+z]=3; break;
						}else{
							z-=p-j-1;
						}
					}else{
						S[j+1]=x;
						B[b++]=j+1; break;
					}
				}
				if(S[p]==1){
					if(x==3){
						S[p]=2; x=1;
						B[b++]=p;
					}else if(x==1){
						S[p]=0; x=1;
					}else{
						if(--z>=0){
							S[p]=0; x=3;
						}else{
							S[p]=3; break;
						}
					}
				}else{
					if(x==3){
						S[p]=1; x=2;
						B[b++]=p;
					}else if(x==2){
						S[p]=0; x=2;
					}else{
						z--;
						if(z>=0){
							S[p]=0; x=3;
						}else{
							S[p]=3; break;
						}
					}
				}
				j=p;
			}
			while(b) A[a++]=B[--b];
		}else if(S[i]!=0){
			A[a++]=i;
		}
	}
	printS();
	printT();
}
