#include<stdio.h>

int main(){
	
	int i,j,p,c;
	int w,h;
	int wf,bf;
	int pw,ph;
	
	while(1){
		scanf("%d%d",&w,&h);
		if(w==0&&h==0) break;
		c = 1;
		wf = 0;
		bf = 0;
		int a[52][52]={};
		for(i=0;i<(w+2)*(h+2);i++){
			a[i/(w+2)][i%(w+2)] = 1;
		}
		char in[w];
		for(i=0;i<h;i++){
			scanf("%s",in);
			for(j=0;j<w;j++){
				if(in[j]=='W'){
					a[i+1][j+1] = 4;
					wf++;
				}else if(in[j]=='B'){
					a[i+1][j+1] = 9;
					bf++;
				}
			}
		}
		while(c!=0){
			c = 0;
			for(p=0;p<w*h;p++){
				pw = p%w+1;
				ph = p/w+1;
				if(a[ph][pw]==0||a[ph][pw]==4||a[ph][pw]==9){
					continue;
				}
				int x = a[ph-1][pw]*a[ph+1][pw]*a[ph][pw-1]*a[ph][pw+1];
				if(x==0){
					a[ph][pw] = 0;
					c++;
					continue;
				}
				int f1 = x%2;
				int f2 = x%3;
				if(a[ph][pw]==1){
					if(f1==0&&f2==0){
						a[ph][pw] = 0;
						c++;
						continue;
					}else if(f1!=0&&f2!=0){
						continue;
					}else if(f1==0){
						a[ph][pw] = 2;
						c++;
						continue;
					}else{
						a[ph][pw] = 3;
						c++;
						continue;
					}
				}
				if(a[ph][pw]==2){
					if(f2==0){
						a[ph][pw] = 0;
						c++;
						continue;
					}
				}
				if(a[ph][pw]==3){
					if(f1==0){
						a[ph][pw] = 0;
						c++;
						continue;
					}
				}
			}
		}
		int wc=0,bc=0;
		for(i=0;i<w*h;i++){
			if(a[i/w+1][i%w+1]==2){
				wc++;
			}
			if(a[i/w+1][i%w+1]==3){
				bc++;
			}
		}
		printf("%d %d\n",bc,wc);
	}
}