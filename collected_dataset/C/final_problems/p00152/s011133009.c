#include <stdio.h>

typedef struct{
	int no;
	int score;
} Info;

void sortInfo(int n,Info t[]){
	int i,j;
	for(i=0;i<n-1;i++){
		for(j=i+1;j<n;j++){
			if(t[i].score < t[j].score ||
			   t[i].score == t[j].score && t[i].no > t[j].no){
				Info tmp = t[i];
				t[i] = t[j];
				t[j] = tmp;
			}
		}
	}
}

int scoring(void){
	int i, x, total, frame, time, sum, flgSpare, flgStrike;
	
	total = sum = 0;
	frame = time = 1;
	flgSpare = flgStrike = 0;
	
	while(frame < 10){
		scanf("%d",&x);
		sum += x;
		
		if(flgSpare){
			total += x;
			flgSpare = 0;
		}
		else if(flgStrike){
			if(flgStrike == 3){
				total += x;
				flgStrike--;
			}
			total += x;
			flgStrike--;
		}
		
		if(time == 2){
			total += sum;
			if(sum == 10){
				flgSpare = 1;
			}
			sum = 0;
			time = 1;
			frame++;
		}
		else if(time == 1){
			time++;
			if(x == 10){
				total += 10;
				flgStrike += 2;
				sum = 0;
				time = 1;
				frame++;
			}
		}
	}
	
	for(i=0;i<3;i++){
		if(i == 2 && sum < 10) break;
		
		scanf("%d",&x);
		total += x;
		sum += x;
		
		if(flgSpare){
			total += x;
			flgSpare = 0;
		}
		else if(flgStrike){
			if(flgStrike == 3){
				total += x;
				flgStrike--;
			}
			total += x;
			flgStrike--;
		}
	}

	return total;
}

int main(void){
	int i, n;
	Info info[40];
	
	while(scanf("%d",&n) && n){
		for(i=0;i<n;i++){
			scanf("%d",&info[i].no);
			info[i].score = scoring();
		}
		sortInfo(n,info);
		for(i=0;i<n;i++){
			printf("%d %d\n",info[i].no,info[i].score);
		}
	}
	
	return 0;
}