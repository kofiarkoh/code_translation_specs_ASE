#include<stdio.h>
#include<stdlib.h>

int compare_int(const void *a,const void *b){
	return *(long long*)a-*(long long*)b;
}

long long ru2(int n){
	int i;
	long long r=1;
	for(i=0;i<n;i++)r=r*2;
	return r;
}

int main(){
	int i,j,k,n,ans=0,f[200000]={0};
	long long a[200000],r2[41];
	for(i=0;i<=40;i++)r2[i]=ru2(i);
	scanf("%d",&n);
	for(i=0;i<n;i++)scanf("%lld",&a[i]);
	qsort(a,n,sizeof(long long),compare_int);
	for(i=n-1;i>=0;i--){
		for(j=0;j<=40;j++){
			if(a[i]<r2[j]&&r2[j]<=2*a[i]&&f[i]==0){
				int lb=-1,ub=n;
				while(ub-lb>1){
					int mid=(ub+lb)/2;
					if(a[mid]>=(r2[j]-a[i]))ub=mid;
					else lb=mid;
				}
				if(a[i]+a[ub]==r2[j]){
					if(f[ub]==0&&ub!=i){
						f[i]=1;
						f[ub]=1;
						ans++;
					}else{
						for(k=ub+1;a[k]==a[ub];k++){
							if(f[k]==0&&k!=i){
								f[k]=1;
								f[i]=1;
								ans++;
								break;
							}
						}
					}
				}
			}
		}
	}
	printf("%d\n",ans);
	return 0;
}