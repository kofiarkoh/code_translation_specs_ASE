#include<stdio.h>
int main(void){
	int mi[1000],mo[1000],li[1000],lo[1000],pcsum,msum,r,q,time,m,sw,ts,te,i,j,k,s,cnt,ans,tx,flag;
	while(1){
		scanf("%d %d",&pcsum,&msum);
		if(pcsum==0&&msum==0){
			break;
		}
		for(i=0;i<1000;i++){
			mi[i]=0;
			mo[i]=0;
		}
		scanf("%d",&r);
		j=k=0;
		while(r!=0){
			scanf("%d %d %d %d",&time,&i,&m,&sw);
			if(sw==1){
				mi[j]=time+m*10000;
				j++;
			}
			else if(sw==0){
				mo[k]=time+m*10000;
				k++;
			}
			r--;	
		}
		scanf("%d",&q);
		while(q!=0){
			scanf("%d %d %d",&ts,&te,&m);
			for(i=0;i<1000;i++){
				li[i]=0;
				lo[i]=0;
			}
			i=0;
			cnt=0;
			while(cnt<=j){
				if(mi[cnt]/10000==m){
					li[i]=mi[cnt]%10000;
					i++;
				}
				cnt++;
			}
			i=0;
			cnt=0;
			while(cnt<=k){
				if(mo[cnt]/10000==m){
					lo[i]=mo[cnt]%10000;
					i++;
				}
				cnt++;
			}
			tx=540;
			flag=0;
			i=s=0;
			ans=0;
			while(tx<=1260){
				if(li[i]==tx){
					flag++;
					i++;
				}
				else if(lo[s]==tx){
					flag--;
					s++;
				}
				if(flag>0&&tx>=ts&&tx<te){
					ans++;
				}
				tx++;
			}
			printf("%d\n",ans);
			q--;
		}
	}
	return 0;
}