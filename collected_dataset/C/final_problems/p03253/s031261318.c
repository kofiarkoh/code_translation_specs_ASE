#include<stdio.h>

#define mod 1000000007

long long calc(long long a,long long b){
	if(b==0)return 1;
	if(b%2==0){
		long long i=calc(a,b/2);
		return (i*i)%mod;
	}
	return (a*calc(a,b-1))%mod;
}

long long ncm(int n,int m){
	int i;
	long long r=1,rr=1;
	if(m>n/2)m=n-m;
	for(i=0;i<m;i++)r=(r*(n-i))%mod;
	for(i=m;i>=1;i--)rr=rr*i;
	return (r*calc(rr,mod-2))%mod;
}

int main(){
	int n,i;
	long long m,yakk[1000][2]={0},k,c=0,ans=1;
	scanf("%d%lld",&n,&m);
	for(k=1;k*k<=m;k++){
		if(k>1&&m%k==0){
			for(i=0;i<c;i++){
				if(yakk[i][0]==k){
					yakk[i][1]++;
					m=m/k;
					k=1;
					break;
				}
			}
			if(i==c){
				yakk[c][0]=k;
				yakk[c][1]=1;
				m=m/k;
				k=1;
				c++;
			}
		}
	}
	//printf("%lld\n",m);
	if(m!=1){
		for(i=0;i<c;i++){
			if(yakk[i][0]==m){
				yakk[i][1]++;
				break;
			}
		}
		if(i==c){
			yakk[c][0]=m;
			yakk[c][1]=1;
			c++;
		}
	}
	//ncm(2,1);
	//for(i=0;i<c;i++)printf("%d\n",yakk[i][0]);
	for(i=0;i<c;i++)ans=(ncm(yakk[i][1]+n-1,yakk[i][1])*ans)%mod;
	printf("%lld\n",ans);
	return 0;
}
