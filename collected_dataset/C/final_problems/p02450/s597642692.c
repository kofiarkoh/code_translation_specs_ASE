#include <stdio.h>
#include <stdlib.h>


void Permutation(char *num_box, char *out_num, int n, int count)
{
	char remove;
	int i;

	if(count == n){
		for(i = 0; i < n; i++){
			printf("%d", out_num[i]);
			if(i != n - 1)
				printf(" ");
		}
		printf("\n");
		return;
	}

	for(i = 0; i < n; i++){
		if(num_box[i] != -1){
			remove = num_box[i];
			num_box[i] = -1;
			out_num[count] = remove;
			Permutation(num_box, out_num, n, count + 1);
			num_box[i] = remove;
		}
	}
}


int main()
{
	int n;
	char temp;
	char *num_box, *out_num;
	int i, j;

	scanf("%d", &n);

	num_box = (char *)malloc(sizeof(char) * n);
	for(i = 0; i < n; i++)
		num_box[i] = i + 1;

	out_num = (char *)malloc(sizeof(char) * n);

	Permutation(num_box, out_num, n, 0);

	return 0;
}
