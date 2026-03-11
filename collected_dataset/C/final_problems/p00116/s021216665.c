#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int i,j,k,h,w,min,ans,mat[700][700];
char c[600];

int main(){
	while(scanf("%d %d",&h,&w)*h*w){
		for(i=0;i<h;i++){
			k=0;
			scanf("%s",c);
			for(j=0;j<w;j++){
				if(c[j]=='.')k++; else k=0;
				mat[i][j]=k;
			}
		}

		ans=0;
		for(j=w-1;j>=0 && (j+1)*h>ans ;j--){
			for(k=h-1;k>=0;k--){
				min=mat[k][j];
				if(min*(h-k)<=ans)continue;
				for(i=1;i+k<h;i++){
					if(min>mat[i+k][j])min=mat[i+k][j];
					if(min*(h-k)<=ans)break;
					if(min*(i+1)>ans)ans=min*(i+1);
				}
			}
		}
		printf("%d\n",ans);
	}
	return 0;
}