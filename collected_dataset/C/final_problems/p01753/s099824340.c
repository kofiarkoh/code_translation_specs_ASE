#include<stdio.h>
#include<math.h>
typedef struct O{int x,y,z,r;long long l;}O;
typedef struct P{int x,y,z;}P;
typedef struct V{double x,y,z;}V;

O obj[51];
P s,t,v;
int N,Q,i,j;
long long res;

int main()
{
	scanf("%d%d",&N,&Q);
	for(i=0;i<N;i++)
		scanf("%d%d%d%d%lld",&obj[i].x,&obj[i].y,&obj[i].z,&obj[i].r,&obj[i].l);
	for(i=0;i<Q;i++)
	{
		res=0;
		scanf("%d%d%d%d%d%d",&s.x,&s.y,&s.z,&t.x,&t.y,&t.z);
		for(j=0;j<N;j++)
		{
			P v={t.x-s.x,t.y-s.y,t.z-s.z};
			double a=v.x*(obj[j].x-s.x)+v.y*(obj[j].y-s.y)+v.z*(obj[j].z-s.z);
			double b=v.x*v.x+v.y*v.y+v.z*v.z;
			double c=a/b;
			V l={s.x+v.x*c,s.y+v.y*c,s.z+v.z*c};
			double len=sqrt(pow(l.x-obj[j].x,2)+pow(l.y-obj[j].y,2)+pow(l.z-obj[j].z,2));
			if(len-obj[j].r<=0)
				res+=obj[j].l;
		}
		printf("%lld\n",res);
	}
	
	return 0;
}