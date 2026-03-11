#define PI 3.141592
#include<stdio.h>
#include<math.h>

int main(){
	int N,i,k;
	scanf("%d",&N);
	while(N != 0){
		int x[N],y[N],r[N],m[N];
		double j;
		for(i=0;i<N;i++){
			scanf("%d %d %d %d",&x[i],&y[i],&r[i],&m[i]);
		}
		int max = 0;
		for(i=0;i<N;i++){
			for(j=0; j< 2*PI; j=j+0.001){
				double a,b,c,ab;
				if(j == 0 || j == PI){
					a = 1.0;
					b = 0.0;
					c = -x[i]-r[i]*cos(j);
					ab = sqrt(pow(a,2.0)+pow(b,2.0));
				}else{
					a = 1 / tan(j);
					if(a < 0.01 && a > -0.01){
						a = 0.0;
					}
					b = 1.0;
					c = (-x[i]-r[i]*cos(j)) / tan(j) - y[i] - r[i] * sin(j);
					if(a == 0){
						c = (int)(c*1000)/1000;
					}
					ab = sqrt(pow(a,2.0)+pow(b,2.0));
				}
				int tempmax = 0;
				for(k=0;k<N;k++){
					if(i==k){
						tempmax++;
					}else{
						double d = fabs(a*x[k] + b*y[k] + c) / ab;
						if(m[k] == 0){
							if((int)(d+0.5) == r[k]){
								tempmax++;
							}
						}else{
							if(d >= r[k] && d <= r[k] + m[k]){
								tempmax++;
							}
						}
					}
				}
				if(max < tempmax){
					max = tempmax;
				}
			}
		}
		printf("%d\n",max);
		scanf("%d",&N);
	}

}