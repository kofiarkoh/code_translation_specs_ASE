#include<stdio.h>
int main(){
	int a[11]={},z[6]={};
	int i,j;
	int aiko,win,lose;
	while(1){
		
		for(i=0;i<5;i++){
			scanf("%d",&a[i]);
			if(a[i]==0) break;
			a[i+5]=a[i];
		}
		if(i!=5) break;
		for(i=0;i<5;i++){
			aiko=0; win=0; lose=0;
			for(j=i+1;j<5+i;j++){
				if(a[i]==1){
					if(a[i]+a[j]==2) aiko++;
					else if(a[i]+a[j]==3) win++;
					else if(a[i]+a[j]==4) lose++;
				}
				else if(a[i]==2){
					if(a[i]+a[j]==4) aiko++;
					else if(a[i]+a[j]==5) win++;
					else if(a[i]+a[j]==3) lose++;
				}
				else if(a[i]==3){
					if(a[i]+a[j]==6) aiko++;
					else if(a[i]+a[j]==5) lose++;
					else if(a[i]+a[j]==4) win++;
				}
			}
			if(win>=1 && lose>=1  || aiko==4) z[i]=3;
			else if(lose==0) z[i]=1;
			else if(win==0) z[i]=2;
		}
		for(j=0;j<5;j++) printf("%d\n",z[j]);
	}
return 0;
}