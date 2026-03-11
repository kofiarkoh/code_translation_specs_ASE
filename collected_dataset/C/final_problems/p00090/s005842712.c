#include <stdio.h>
#include <math.h>
double pot[100][3],xp,yp,xq,yq,d,a,ans;
int i,j,n;
void check(double x,double y,int p,int q){
	int i; double px,py,a=2.0;
	for(i=0;i<n;i++){
		if(i!=p && i!=q){
			px=pot[i][0];py=pot[i][1];
			if((px-x)*(px-x)+(py-y)*(py-y)<=1)a+=pot[i][2];
		}
	}
	if(a>ans)ans=a;
}
int main(void){
	while(scanf("%d",&n)*n){
		ans=1;i=0;
		for(;n>0;n--){
			scanf("%lf,%lf",&xp,&yp);
			pot[i][2]=0;
			for(j=0;j<i;j++){if(xp==pot[j][0] && yp==pot[j][1])break;}
			pot[j][2]++;if(pot[j][2]>ans)ans=pot[j][2];
			if(i==j){i++;pot[j][0]=xp;pot[j][1]=yp;}else return 1;
		}
		n=i;

		for(i=0;i<n-1;i++){
			xp=pot[i][0];yp=pot[i][1];
			for(j=i+1;j<n;j++){
				xq=pot[j][0];yq=pot[j][1];
				d=(xp-xq)*(xp-xq)+(yp-yq)*(yp-yq);
				if(d==4)check((xp+xq)/2,(yp+yq)/2,i,j);
				else if (d<4){
					d=sqrt(1-d/4);
					a=atan2(xp-xq,yp-yq)+atan(1.0)*2;
					check((xp+xq)/2+d*cos(a),(yp+yq)/2+d*sin(a),i,j);
					check((xp+xq)/2-d*cos(a),(yp+yq)/2-d*sin(a),i,j);
				}
			}
		}
		printf("%d\n",(int)ans);
	}
	return 0;
}