#include<stdio.h>
int P[100];
int A[101];
int main(void){
    int n;
    int i,j,ans;

    scanf("%d",&n);
    for(i=0;i<n;i++){
	scanf("%d",&P[i]);
	for(j=P[i];j>=0;j--){
	    A[j]++;
	}
    }

    ans=0;
    for(i=0;i<=100;i++){
	if(A[i]>=i) ans=i;
    }
    printf("%d\n",ans);

    return 0;
}