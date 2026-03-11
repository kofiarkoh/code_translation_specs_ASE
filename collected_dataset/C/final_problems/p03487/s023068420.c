#include <stdio.h>

int main(void)
{
   int N;
   int A[100001];
   int B[100001]={0,};
   scanf("%d",&N);
   int n;
   int ans=0;
   for(n=0;n<N;n++){
   	scanf("%d",&A[n]);
   	if(A[n]<=N){
   		B[A[n]]++;
   	}else{
   		ans++;
   	}
   }
   for(n=0;n<N;n++){
   	if(B[n+1]>0&&B[n+1]<=100000&&B[n+1]!=n+1){
   		if(B[n+1]>n+1){
   		ans+=B[n+1]-(n+1);
   		}else{
   			ans+=B[n+1];
   		}
   	}
   }
   printf("%d\n",ans);
	return 0;
}