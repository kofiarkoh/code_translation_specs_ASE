#include <stdio.h>
#include <string.h>

#define COMBI_NO (1 * 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10)


char table[COMBI_NO][9];
int table_no;
int use[10];

void make_table(int depth)
{
	int i;

	if (depth == 0){
		table_no++;
		return;
	}
	
	for (i = 0; i < 10; i++){
		if (use[i] == 0){
			use[i] = 1;
			table[table_no][depth - 1] = i;
			make_table(depth - 1);
		 	use[i] = 0;
		}
	}
}


int main(void)
{
	int i, j;
	int n, s;
	int sum;
	int cnt;
	
	memset(table, 0xff, sizeof(table));
	make_table(9);

	
	for (i = 0; i < COMBI_NO; i++){
		for (j = 0; j < 9; j++){
			if (table[i][j] == -1){
				table[i][j] = table[i - 1][j];
			}
		}
	}
#if 0
	for (i = 0; i < COMBI_NO; i++){
		for (j = 0; j < 9; j++){
			printf("%d", table[i][j]);
		}
		printf("\n");
	}
#endif
	while (scanf("%d%d", &n, &s) != EOF){
		cnt = 0;
		
		for (i = 0; i < COMBI_NO; i++){
			sum = 0;
			for (j = 1; j <= n; j++){
				if (j < 10){
					sum += j * table[i][j];
				}
				else {
					sum += j * (45 - table[i][0] - table[i][1] - table[i][2] - table[i][3] - table[i][4] -
								     table[i][5] - table[i][6] - table[i][7] - table[i][8] - table[i][9]);
				}
			}
			if (sum == s){
				cnt++;
			}
		}

		for (i = 2; i <= 10 - n; i++){
			cnt /= i;
		}
		
		
		printf("%d\n", cnt);

	}
	return (0);
}