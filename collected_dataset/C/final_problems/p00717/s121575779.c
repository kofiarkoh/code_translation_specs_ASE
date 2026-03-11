#include <stdio.h>
#include <math.h>
#include <stdlib.h>

int main()
{
    int n,t,m0,m,xp[10],yp[10],x[10],y[10],i,k,j,xp0,yp0,l,same,samen;
	
	while(0<=scanf("%d",&n)){
		
		if(n==0){
			break;
		}
		
		for(i=0;i<10;i++){
			xp[i]=0;
			yp[i]=0;
			x[i]=0;
			y[i]=0;
		}
		
		scanf("%d",&m0);
		for(i=0;i<m0;i++){
			scanf("%d%d",&xp[i],&yp[i]);
		}
		
		xp0=xp[0];
		yp0=yp[0];
		
		samen=0;
		
		for(i=0;i<m0;i++){
			
			xp[i]=xp[i]-xp0;
			yp[i]=yp[i]-yp0;
		}
			
			
		
		for(i=1;i<=n;i++){
			scanf("%d",&m);
			if(m!=m0){
				continue;
			}
			
			for(k=0;k<10;k++){
				x[k]=0;
				y[k]=0;
			}
			
			for(k=0;k<m;k++){
				scanf("%d%d",&x[k],&y[k]);
			}
			
			for(k=0;k<2;k++){
				xp0=x[0];
				yp0=y[0];
			
				for(j=0;j<m;j++){
					x[j]-=xp0;
					y[j]-=yp0;
				}
				for(l=0;l<4;l++){
					same=0;
					
					for(j=0;j<m;j++){
						if(x[j]==xp[j] && y[j]==yp[j]){
							++same;
						}
						if (same==m){
							printf("%d\n",i);
						}
					}
					
					for(j=0;j<10;j++){
						t=-y[j];
						y[j]=x[j];
						x[j]=t;
					}
					
				}
				
				for(l=0;l<(m-1)/2;l++){
					j=x[l];
					x[l]=x[m-1-l];
					x[m-1-l]=j;
					j=y[l];
					y[l]=y[m-1-l];
					y[m-1-l]=j;
				}
			}
		}
		printf("+++++\n");
	}
	
	return 0;
}