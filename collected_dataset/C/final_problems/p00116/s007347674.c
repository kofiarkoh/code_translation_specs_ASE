#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int i,j,k,h,w,min,ans,mat[700][700],max[700][700];
char c;

int main(){
	while(scanf("%d %d",&h,&w)*h*w){
		for(i=0;i<h;i++){
			k=0;
			scanf("\n");
			for(j=0;j<w;j++){
				scanf("%c",&c);
				if(c=='.')k++; else k=0;
				mat[i][j]=k;
			}
		}

		ans=0;
		for(j=w-1;j>=0;j--){
			for(k=h-1;k>=0;k--){
				min=mat[k][j];
				for(i=1;i+k<h;i++){
					if(min>mat[i+k][j])min=mat[i+k][j];
					if(min==0)break;
					if(min*(i+1)>ans)ans=min*(i+1);
				}
			}
		}
		printf("%d\n",ans);
	}
	return 0;
}