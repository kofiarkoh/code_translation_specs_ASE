#include<stdio.h>
#include<stdlib.h>

int compare_int(const void *a,const void *b){
		return *(long long*)a-*(long long*)b;
}

int main(){
		int n,i;
		long long a[100000],ans[2];ans[1]=-1000000000000;
		scanf("%d",&n);
		for(i=0;i<n;i++)scanf("%lld",&a[i]);
		qsort(a,n,sizeof(long long),compare_int);
		ans[0]=a[n-1];
		for(i=0;i<n-1;i++){
				long long tmp=a[i];
				if(abs(tmp-ans[0]/2)<abs(ans[1]-ans[0]/2))ans[1]=a[i];
				if(abs(tmp-ans[0]/2-1)<abs(ans[1]-ans[0]/2-1))ans[1]=a[i];
		}
		printf("%lld %lld\n",ans[0],ans[1]);
		return 0;
}
