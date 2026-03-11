#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int i,j,k,n,m,sss,sssss,dist[200];
char ppp,seat[200],next[200];


int main(){
	while(scanf("%d %d%*c",&n,&m)*n){
		for(k=0;k<n;k++){
			seat[k]='#';
			next[k]='#';
			dist[k]=99999;
		}
		seat[n]='\0';

		for(k=0;k<m;k++){
			scanf("%c%*c",&ppp);

			sss=-1;sssss=-1;
			switch(ppp){
				case 'A':
					for(i=0;i<n;i++){if(seat[i]=='#'){sss=i;break;}}
				break;
				case 'B':
					for(i=n-1;i>=0;i--){
						if(seat[i]=='#' && next[i]!='A'){sss=i;break;}
						if(sssss<0 && seat[i]=='#')sssss=i;
					}
					if(sss==-1)sss=sssss;
				break;
				case 'C':
					if(k==0)sss=n/2;
					else{
						for(i=0;i<n;i++){
							if(seat[i]!='#'){
								if(i+1<n && seat[i+1]=='#'){sss=i+1;break;}
								if(i-1>=0 && seat[i-1]=='#'){sss=i-1;break;}
							}
						}
					}
				break;
				case 'D':
					sssss=0;
					for(i=0;i<n;i++){
						if(dist[i]>sssss){sss=i;sssss=dist[i];}
					}
				break;
			}

			seat[sss]=ppp;
			if(sss>0)next[sss-1]=ppp;
			next[sss+1]=ppp;
			for(i=0;sss-i>=0;i++){if(dist[sss-i]>i)dist[sss-i]=i; else break;}
			for(i=1;sss+i<n;i++){if(dist[sss+i]>i)dist[sss+i]=i; else break;}
		}
		
		printf("%s\n",seat);
	}
	return 0;
}