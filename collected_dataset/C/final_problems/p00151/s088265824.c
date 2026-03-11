#include <stdio.h>

int main() {
	char grid[255][256];
	int n;
	int i,j,k;
	int ones;
	int max_ones;
	
	while (1) {
		scanf("%d",&n);
		if (n == 0) {
			break;
		}
		for (i = 0;i != n;++i) {
			scanf("%s",grid[i]);
		}
		max_ones = 0;
		// vertical
		for (ones = 0,i = 0;i != n;++i) {
			for (j = 0;j != n;++j) {
				if (grid[i][j] == '1') {
					++ones;
				} else {
					if (ones > max_ones) {
						max_ones = ones;
					}
					ones = 0;
				}
			}
			if (ones > max_ones) {
				max_ones = ones;
			}
			ones = 0;
		}
		// horizontal
		for (ones = 0,j = 0;j != n;++j) {
			for (i = 0;i != n;++i) {
				if (grid[i][j] == '1') {
					++ones;
				} else {
					if (ones > max_ones) {
						max_ones = ones;
					}
					ones = 0;
				}
			}
			if (ones > max_ones) {
				max_ones = ones;
			}
			ones = 0;
		}
		// diagonal
		for (ones = 0,j = 0;j != n;++j) {
			for (i = 0,k = j;k != -1;++i,--k) {
				if (grid[i][k] == '1') {
					++ones;
				} else {
					if (ones > max_ones) {
						max_ones = ones;
					}
					ones = 0;
				}
			}
			if (ones > max_ones) {
				max_ones = ones;
			}
			ones = 0;
		}
		for (ones = 0,i = 1;i != n;++i) {
			for (k = i,j = n-1;k != n;++k,--j) {
				if (grid[k][i] == '1') {
					++ones;
				} else {
					if (ones > max_ones) {
						max_ones = ones;
					}
					ones = 0;
				}
			}
			if (ones > max_ones) {
				max_ones = ones;
			}
			ones = 0;
		}
		for (ones = 0,j = 0;j != n;++j) {
			for (i = 0,k = j;k != n;++i,++k) {
				if (grid[i][k] == '1') {
					++ones;
				} else {
					if (ones > max_ones) {
						max_ones = ones;
					}
					ones = 0;
				}
			}
			if (ones > max_ones) {
				max_ones = ones;
			}
			ones = 0;
		}
		for (ones = 0,i = 1;i != n;++i) {
			for (k = i,j = 0;k != n;++k,++j) {
				if (grid[k][i] == '1') {
					++ones;
				} else {
					if (ones > max_ones) {
						max_ones = ones;
					}
					ones = 0;
				}
			}
			if (ones > max_ones) {
				max_ones = ones;
			}
			ones = 0;
		}
		printf("%d\n",max_ones);
	}
	return 0;
}