#include <stdio.h>
int main()
{
		int N,K,A[200000],i,j,tmp;
		int ans=0,count[200000];
		scanf("%d", &N);
		scanf("%d", &K);
		for (i = 0; i < N; i++)scanf("%d",&A[i]);
		for (i = 0; i < N; i++)count[A[i]] +=1;
			for(i=0;i<N;i++)
			  for(j=i+1;j<N;j++)
			  if(count[i]<=count[j]){
			      	tmp = count[i];
					count[i] = count[j];
					count[j] = tmp;
			  }
			  for(i=0;i<K;i++)ans+=count[i];
			   ans=N-ans;
			  printf("%d",ans);
		return 0;
}