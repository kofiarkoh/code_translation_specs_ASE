#include<stdio.h>

int a[55][55],b[55][55],c[55][55];
int main(){
	int i,j,k,l,H,W,h,w,ans,sum;
	
	scanf("%d%d",&H,&W);
	for(i=0;i<H;i++)for(j=0;j<W;j++)scanf("%d",&a[i][j]);
	for(i=0;i<H;i++)for(j=0;j<W;j++)scanf("%d",&b[i][j]);
	scanf("%d%d",&h,&w);
	for(i=0;i<h;i++)for(j=0;j<w;j++)scanf("%d",&c[i][j]);
	
	ans=-10000000;
	for(i=0;i<=H-h;i++){
		for(j=0;j<=W-w;j++){
			for(k=0;k<h;k++){
				for(l=0;l<w;l++)if(b[i+k][j+l]!=c[k][l])break;
				if(l!=w)break;
			}
			if(k!=h)continue;
			sum=0;
			for(k=0;k<h;k++)for(l=0;l<w;l++)sum+=a[i+k][j+l];
			if(ans<sum)ans=sum;
		}
	}
	if(ans==-10000000)printf("NA\n");
	else printf("%d\n",ans);
	
	
	return 0;
}
