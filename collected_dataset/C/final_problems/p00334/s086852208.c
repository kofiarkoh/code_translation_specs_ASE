#include<stdio.h>
int main(void){
	int n,p[1000][3],i,o=0,r,f,e,j;
	scanf("%d",&n);
	for(i=0;i<n;i++){
		scanf("%d%d%d",&p[i][0],&p[i][1],&p[i][2]);
		for(r=0;r<3;r++){
			for(e=r+1;e<3;e++){
				if(p[i][r]>p[i][e]){
					f=p[i][r];
					p[i][r]=p[i][e];
					p[i][e]=f;
				}
			}
		}
	}
	for(i=0;i<n;i++){
		for(j=0;j<n;j++){
			if(j==i){
				j++;
			}
			if(p[i][0]==p[j][0]&&p[i][1]==p[j][1]&&p[i][2]==p[j][2]&&p[j][0]!=0){
				o++;
				p[j][0]=0;
			}
		}
	}
	printf("%d\n",o);
	return 0;
}