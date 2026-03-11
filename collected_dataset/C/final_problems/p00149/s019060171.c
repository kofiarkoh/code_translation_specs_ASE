#include<stdio.h>
int main()
{
	double l,r;
	int al=0,bl=0,cl=0,dl=0,ar=0,br=0,cr=0,dr=0;
	while(scanf("%lf%lf",&l,&r)!=EOF){
		if(l>=1.1){
			al++;
		}else if(l>=0.6){
			bl++;
		}else if(l>=0.2){
			cl++;
		}else{
			dl++;
		}
		if(r>=1.1){
			ar++;
		}else if(r>=0.6){
			br++;
		}else if(r>=0.2){
			cr++;
		}else{
			dr++;
		}
	}
	printf("%d %d\n%d %d\n%d %d\n%d %d\n",al,ar,bl,br,cl,cr,dl,dr);
	return 0;
}