#include<stdio.h>
int main(){
	int N,C,ic,i,i2,or,t,p,m,team[100001],point[100001],work;
	scanf("%d%d",&N,&C);
	for(i=1;i<=N;i++){
		team[i]=i;
		point[i]=0;
	}
	for(ic=1;ic<=C;ic++){
		scanf("%d",&or);
		if(or==0){
			scanf("%d%d",&t,&p);
			for(i=1;i<=N;i++)if(team[i]==t)break;
			point[i]+=p;
		}
		else{
			scanf("%d",&m);
			//???????????????
			for(i=1;i<N;i++){
				for(i2=i+1;i2<=N;i2++){
					if(point[i]<point[i2]){
						work=point[i];
						point[i]=point[i2];
						point[i2]=work;
						work=team[i];
						team[i]=team[i2];
						team[i2]=work;					}
					else if(point[i]==point[i2]){
						if(team[i]>team[i2]){
							work=team[i];
							team[i]=team[i2];
							team[i2]=work;
						}
					}
				}
			}
			//???????????§????????????
			printf("%d %d\n",team[m],point[m]);
		}
	}
}