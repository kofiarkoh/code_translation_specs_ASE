#include<stdio.h>
static const int N_MAX=100100;
int main(void){
    int n,A[N_MAX],L[N_MAX];
    int i,j,k,max,ans;

    scanf("%d",&n);
    A[0]=0;
    for(i=1;i<=n;i++){
	scanf("%d",&A[i]);
    }

    for(i=1;i<=n;i++){
	max=0;
	for(j=n-1;j>=0;j--){
	    if(A[j]<A[i] && L[j]>max){
		max=L[j];
	    }
	}
	L[i]=max+1;
    }

    ans=0;
    for(i=1;i<=n;i++){
	if(L[i]>ans) ans=L[i];
    }

    printf("%d\n",ans);

    return 0;
}