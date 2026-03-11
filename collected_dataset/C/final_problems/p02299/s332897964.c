#include<stdio.h>
#include<math.h>

typedef struct{
	double x;
	double y;
}POINT;

double dot(POINT a,POINT b){//??????
	return a.x*b.x + a.y*b.y;
}
double cross(POINT a,POINT b){//??????
	//return (a.x*a.x + a.y*a.y)*(b.x*b.x + b.y*b.y) - dot(a,b)*dot(a,b);
    return a.x*b.y - a.y*b.x;
}
POINT make_vec(POINT a,POINT b){//a->b
	POINT ans;
	ans.x = b.x-a.x;
	ans.y = b.y-a.y;
	return ans;
}

int main(){
	int i,j,k,l,m,n;
	double w,e,r,t,y;
	double rad;
	POINT g[101];//n
	POINT q[10000];//m
	int ans[10000] = {0};

	scanf("%d",&n);
	for(i = 0;i < n;i++){
		scanf("%lf%lf",&g[i].x,&g[i].y);
	}
	g[n] = g[0];
	scanf("%d",&m);
	for(i = 0;i < m;i++){
		scanf("%lf%lf",&q[i].x,&q[i].y);
		for(j = 0;j < n;j++){
			if(g[j].x == q[i].x && g[j].y == q[i].y){
				ans[i] = 1;
			}
		}
	}

	for(j = 0;j < m;j++){
		if(ans[j] == 1)continue;
rad =0;
		for(i = 0;i < n;i++){
			w = atan2(cross(
				make_vec(q[j],g[i]),
				make_vec(q[j],g[i+1]))
				,dot(
				make_vec(q[j],g[i]),
				make_vec(q[j],g[i+1]))
				);//q[j],g[i],g[i+1]
			rad += w;
//printf("%lf\n",w);
			if(fabs(w - 3.1415926535)<=0.000001 || fabs(w +3.1415926535)<=0.000001)ans[j] = 1;
		}
//printf("\n%lf\n\n",rad);
		if(ans[j] != 1){
			if(fabs(rad - 3.1415926535 * 2)<=0.000001){//2??
				ans[j] = 2;//in!
			}
		}
	}
	for(i = 0;i < m;i++)printf("%d\n",ans[i]);
	return 0;
}