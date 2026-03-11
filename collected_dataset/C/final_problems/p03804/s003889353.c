#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <ctype.h>
#include <stdint.h>
#include <string.h>
#include <wchar.h>

#define N_MAX (100)
#define P_MAX (100)
#define DP_ARRAY_SIZE (N_MAX * P_MAX / 32 + 1)

#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define ABS(a) ((a) < 0 ? -(a) : (a))
#define ABSS(a, b) ((a) > (b) ? (a) - (b) : (b) - (a))

int compare_f(const void *a, const void *b) {
	return *((int64_t *)a) < *((int64_t *)b) ? -1 : 1;
}

int compare_f2(const void *a, const void *b) {
	return *((uint64_t *)a) > *((uint64_t *)b) ? -1 : 1;
}

static size_t comb(const size_t n, const size_t r) {
	size_t result = 1;

	for (size_t i = 0; i < r; i++) {
		result *= n - i;
		result /= i + 1;
	}

	return result;
}

static uint64_t combU64(const uint64_t n, const uint64_t r) {
	uint64_t result = 1;

	for (uint64_t i = 0; i < r; i++) {
		result *= n - i;
		result /= i + 1;
	}

	return result;
}

static uint64_t gcd(uint64_t m, uint64_t n)
{
	uint64_t temp;

	while (m%n != 0) {
		temp = n;
		n = m % n;
		m = temp;
	}

	return n;
}

static char s[200001];

int main(void) {
	size_t N, M;
	char A[50][51], B[50][51];

	scanf("%zu %zu\n", &N, &M);
	for (size_t i = 0; i < N; i++) {
		scanf("%s\n", A[i]);
	}
	for (size_t i = 0; i < M; i++) {
		scanf("%s", B[i]);
		if (i < M - 1) {
			scanf("\n");
		}
	}

	bool found = false;

	for (size_t rowOffset = 0; rowOffset <= N - M; rowOffset++) {
		for (size_t colOffset = 0; colOffset <= N - M; colOffset++) {
			bool matched = true;
			for (size_t row = 0; row < M; row++) {
				for (size_t col = 0; col < M; col++) {
					if (A[rowOffset + row][colOffset + col] != B[row][col]) {
						matched = false;
						break;
					}
				}

				if (matched) {
					found = true;
					break;
				}
			}
		}
	}

	puts(found ? "Yes" : "No");

	return 0;
}
