#include <stdio.h>
#include <limits.h>
int main(int argc, char *argv[])
{
	long long n,k,i,det,j,sum,max=INT_MIN,q,l,z;
	scanf("%lld%lld",&n,&k);
	long long p[10000],c[10000],v;
	long long g;
	for(i=1;i<=n;i++)
		scanf("%lld",&p[i]);
	for(i=1;i<=n;i++)
		scanf("%lld",&c[i]);
	for(i=1;i<=n;i++){
		long long pref[10000];
		det=0;sum=0;l=p[i];
		v=1;
		for(j=i,z=0;(p[j]!=l||v)&&det<k&&det<10000;){
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
		if(z)
		go=pref[z-1];
		for(q=0;q<z;q++)
			pref[q]+=(kk/det)*go;
			for(q=0;q<(kk%det);q++){
			pref[q]+=go;

			}
		}
		rr:
			for(q=0;q<z;q++)
				if(pref[q]>max)
					max=pref[q];
	}
	printf("%lld\n",max);
}