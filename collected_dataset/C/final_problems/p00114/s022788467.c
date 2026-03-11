#include <stdio.h>

int count(int a,int m){
	int x = 1,cnt = 0;
	
	do{
		x = (a*x) % m;
		cnt++;
	}while(x-1);
	
	return cnt;
}

int gcd(int a,int b){
	return b ? gcd(b,a%b) : a;
}

int lcd(int num[3]){
	int i,j,flag,tmp,ware = 1;

	do{
		flag = 0;
		
		for(i = 0;i < 3;i++){
			tmp = gcd(num[i],num[(i+1)%3]);
			if(tmp-1){
				flag = 1;
				ware *= tmp;
				for(j = 0;j < 3;j++){
					if(!(num[j] % tmp)){
						num[j] /= tmp;
					}
				}
			}
		}
	}while(flag);
	
	for(i = 0;i < 3;i++){
		ware *= num[i];
	}
	
	return ware;
}

int main(void) {
	int i;
	int a[3],m[3],cnt[3];
	
	while(1){
		for(i = 0;i < 3;i++){
			scanf("%d%d",&a[i],&m[i]);
		}
		if(!a[0]) break;
		
		for(i = 0;i < 3;i++){
			cnt[i] = count(a[i],m[i]);
		}
		
		printf("%d\n",lcd(cnt));
	}
	
	return 0;
}