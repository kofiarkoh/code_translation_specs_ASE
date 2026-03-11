#include <stdio.h>

int sosuu_num;
int sosuu_list[1000000];

void make_sosuu_list(void) {
	char is_sosuu[1000001];
	int i,j;
	for(i=0;i<=1000000;i++)is_sosuu[i]=1;
	is_sosuu[0]=is_sosuu[1]=0;
	sosuu_num=0;
	for(i=2;i<=1000000;i++) {
		if(is_sosuu[i]) {
			sosuu_list[sosuu_num++]=i;
			for(j=i+i;j<=1000000;j+=i)is_sosuu[j]=0;
		}
	}
}

int main(void) {
	int a,b;
	int i;
	int first;
	int a_sum,b_sum;
	make_sosuu_list();
	while(1) {
		scanf("%d%d",&a,&b);
		if(a==0 && b==0)break;
		a_sum=0;first=1;
		for(i=sosuu_num-1;i>=0;i--) {
			if(a%sosuu_list[i]==0) {
				if(first){a_sum=sosuu_list[i];first=0;}
				else a_sum-=sosuu_list[i];
			}
		}
		b_sum=0;first=1;
		for(i=sosuu_num-1;i>=0;i--) {
			if(b%sosuu_list[i]==0) {
				if(first){b_sum=sosuu_list[i];first=0;}
				else b_sum-=sosuu_list[i];
			}
		}
		puts(a_sum>=b_sum?"a":"b");
	}
	return 0;
}