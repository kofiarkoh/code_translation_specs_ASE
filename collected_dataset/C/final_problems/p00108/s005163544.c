#include <stdio.h>

#define MAX (12)

int search_seq(int *seq, int len, int num);
int numcmp(int *seq_s, int *seq_c, int len);

int main(void)
{
	int now_seq[MAX];
	int af_seq[MAX];
	int count;
	int len;
	int i;
	
	while (1){
		scanf("%d", &len);
		if (len == 0){
			break;
		}
		
		for (i = 0; i < len; i++){
			scanf("%d", &now_seq[i]);
		}
		
		for (i = 0; i < len; i++){
			af_seq[i] = search_seq(now_seq, len, now_seq[i]);
		}
		
		for (count = 0; numcmp(now_seq, af_seq, len) == 0; count++){
			for (i = 0; i < len; i++){
				now_seq[i] = af_seq[i];
			}
			
			for (i = 0; i < len; i++){
				af_seq[i] = search_seq(now_seq, len, now_seq[i]);
			}
		}
		
		printf("%d\n", count);
		for (i = 0; i < len; i++){
			printf("%d%c", now_seq[i], (i == len - 1) ? '\n' : ' ');
		}
	}
	
	return (0);
}

int search_seq(int *seq, int len, int num)
{
	int i;
	int count;
	
	count = 0;
	for (i = 0; i < len; i++){
		if (seq[i] == num){
			count++;
		}
	}
	
	return (count);
}

int numcmp(int *seq_s, int *seq_c, int len)
{
	int i;
	
	for (i = 0; seq_s[i] == seq_c[i] && i < len; i++);
	
	if (i == len){
		return (1);
	}
	
	return (0);
}