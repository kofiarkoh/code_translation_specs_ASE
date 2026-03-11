#include <stdio.h>
#include <limits.h>
int main(int argc, char *argv[])
{
	long long n,k,i,det,j,sum,max=INT_MIN,q,l,z;
	scanf("%lld%lld",&n,&k);
	long long p[n+1],c[n+1],v;
	long long g;
	for(i=1;i<=n;i++)
		scanf("%lld",&p[i]);
	for(i=1;i<=n;i++)
		scanf("%lld",&c[i]);
	for(i=1;i<=n;i++){
		long long pref[1000]={0};
		det=0;sum=0;l=p[i];
		v=1;
		for(j=i,z=0;(p[j]!=l||v)&&det<k;){
			det++;
			sum+=c[p[j]];
			pref[z++]=sum;
			j=p[j];
		}
		if(det==k)
			goto rr;
		else{
			long long kk=k,go;
			kk-=det;
		//	sum+=(k/det)*sum;
		go=pref[z-1];
		for(q=0;q<z;q++)
			pref[q]+=(kk/det)*go;
			for(q=0;q<(kk%det);q++){
			//	sum+=c[p[j]];
			pref[q]+=go;
			//	j=p[j];
			}
		}
		rr:
//		if(sum<0)sum=sum*(-1);
//		printf("%lld\n",sum);
	//	if(sum>max)
			//max=sum;
			for(q=0;q<z;q++)
				if(pref[q]>max)
					max=pref[q];
	}
	printf("%lld\n",max);
}