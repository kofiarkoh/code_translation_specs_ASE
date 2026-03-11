#include <stdio.h>
#include <stdlib.h>
#define max(p,q)((p)>(q)?(p):(q))
#define bit(n,m)(((n)>>(m))&1)

long vv[1<<20],ww[1<<20];
int idx[1<<20];
int cnt;
int c(const void*p,const void*q){
	if(ww[*(int*)p]<ww[*(int*)q])return -1;
	if(ww[*(int*)p]>ww[*(int*)q])return  1;
	if(vv[*(int*)p]>vv[*(int*)q])return -1;
	return 1;
}

int bs(long x){
	//ww[idx[i]]<=xとなるi
	int l=0,r=cnt;
	while(r-l>1){
		int m=(l+r)/2;
		if(ww[idx[m]]<=x)l=m;
		else r=m;
	}
	return l;
}

long v[50],w[50];
int main(){
	long n,m;
	scanf("%ld%ld",&n,&m);
	for(int i=0;i<n;i++)scanf("%ld%ld",v+i,w+i);
	int N=n-n/2;
	n/=2;
	for(int i=0;i<1<<n;i++){
		idx[i]=i;
		for(int k=0;k<n;k++)if(bit(i,k)){
			vv[i]+=v[k];
			ww[i]+=w[k];
		}
	}
	qsort(idx,1<<n,sizeof(int),c);
	cnt=1;
	long cv=vv[idx[0]];
	for(int i=1;i<1<<n;i++){
		if(vv[idx[i]]>cv){
			cv=vv[idx[i]];
			idx[cnt++]=idx[i];
		}
	}
	
//	for(int i=0;i<cnt;i++)printf("%ld %ld\n",ww[idx[i]],vv[idx[i]]);
	
	long ans=0;
	for(int i=0;i<1<<N;i++){
		long vsum=0,wsum=0;
		for(int k=0;k<N;k++)if(bit(i,k)){
			vsum+=v[n+k];
			wsum+=w[n+k];
		}
		if(wsum<=m){
			int j=bs(m-wsum);
			ans=max(ans,vsum+vv[idx[j]]);
		}
	}
	printf("%ld\n",ans);
}
