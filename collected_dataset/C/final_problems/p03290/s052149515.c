#include <stdio.h>
#define bit(n,m)(((n)>>(m))&1)
#define min(p,q)((p)<(q)?(p):(q))

int cnt[11];
int bo[11];
int main(){
	int n,s;
	scanf("%d%d",&n,&s);
	int ans=1e9;
	for(int i=0;i<n;i++)scanf("%d%d",cnt+i,bo+i);
	for(int i=0;i<1<<n;i++){
		int temps=0,tempcnt=0;
		for(int k=0;k<n;k++)if(bit(i,k)){
			temps+=cnt[k]*(k+1)*100+bo[k];
			tempcnt+=cnt[k];
		}
		if(temps>=s){
			ans=min(ans,tempcnt);
		}else{
			int rest=s-temps;
			for(int k=0;k<n;k++)if(!bit(i,k)){
				if(rest<=(cnt[k]-1)*(k+1)*100){
					ans=min(ans,tempcnt+(rest+(k+1)*100-1)/((k+1)*100));					
				}
			}
		}
	}	
	printf("%d",ans);
}