#include <stdio.h>

typedef struct {
	int connected;
	int distance[100];
}onesiseki;

int main(void) {
	onesiseki siseki[100];
	int sisekinum;
	int i,j;
	int roadnum;
	int a,b,dist;
	int distancesum;
	int minsiseki,minsisekito,mindist;
	while(1) {
		scanf("%d",&sisekinum);
		if(sisekinum==0)break;
		for(i=0;i<sisekinum;i++) {
			siseki[i].connected=0;
			for(j=0;j<sisekinum;j++)siseki[i].distance[j]=-1;
		}
		scanf("%d",&roadnum);
		for(i=0;i<roadnum;i++) {
			scanf("%d,%d,%d",&a,&b,&dist);
			siseki[a].distance[b]=dist;
			siseki[b].distance[a]=dist;
		}
		siseki[0].connected=1;
		distancesum=0;
		while(1) {
			minsiseki=minsisekito=mindist=-1;
			for(i=0;i<sisekinum;i++) {
				if(siseki[i].connected) {
					for(j=0;j<sisekinum;j++) {
						if(siseki[j].connected==0 && siseki[i].distance[j]>0) {
							if(mindist<0 || siseki[i].distance[j]<mindist) {
								mindist=siseki[i].distance[j];
								minsiseki=i;
								minsisekito=j;
							}
						}
					}
				}
			}
			if(minsiseki==-1)break;
			distancesum+=mindist-100;
			siseki[minsisekito].connected=1;
		}
		printf("%d\n",distancesum/100);
	}
	return 0;
}