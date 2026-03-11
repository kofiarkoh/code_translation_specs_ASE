#include <stdio.h>

int main(void){
	while(1){
		//input
		int b;
		scanf("%d", &b);
		//scanf("%d", &b) // AOJではscanf_s()が使えないことに注意
		if(!b) break;

		//processing
		//できるだけ多くのフロアを借りたい -> 安い部屋から順。
		int high=1; //借りるフロアの中での最上階
		int low=1;  //借りるフロアの中での最下層
		int sum=1;  //lowからhighまでの和
		while(1){
			if(sum == b) break; //予算通りになる借り方を発見。ループを抜け出力へ
			else if(sum < b){
				//予算に余裕がありそうなので、さらに上の部屋を借りてみる。
				high++;
				sum += high;
			} else{
				//予算をオーバーしているので、最下層を諦める。
				sum -= low;
				low++;
			}
		}

		//output
		int ans1 = low; //借りるフロアの中での最下層
		int ans2 = high - low + 1; //借りるフロアの総数
		printf("%d %d\n", ans1, ans2);
	}
	return 0;
}
