#include <stdio.h>

int main(void) {
	int i,j;
	int query1,query2;
	int now,min;
	int array[4];
	int ii,jj,temp;
	while(1) {
		scanf("%d%d",&query1,&query2);
		if(query1==0 && query2==0)break;
		min=0x10000000;
		for(i=1;i*i<=query1;i++) {
			if(query1%i!=0)continue;
			for(j=1;j*j<=query2;j++) {
				if(query2%j!=0)continue;
				array[0]=i;
				array[1]=query1/i;
				array[2]=j;
				array[3]=query2/j;
				for(ii=0;ii<4;ii++) {
					for(jj=ii;jj+1<4;jj++) {
						if(array[jj]<array[jj+1]) {
							temp=array[jj];
							array[jj]=array[jj+1];
							array[jj+1]=temp;
						}
					}
				}
				now=0;
				for(ii=1;ii<4;ii++) {
					now+=(array[ii-1]-array[ii])*(array[ii-1]-array[ii]);
				}
				if(now<0)return 1;
				if(now<min)min=now;
			}
		}
		printf("%d\n",min);
	}
	return 0;
}