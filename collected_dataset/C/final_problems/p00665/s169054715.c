#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <inttypes.h>

struct kyarakutaa_t {
	char name[12];
	int x;
};

int cmp_name(const void* x, const void* y) {
	return strcmp(((const struct kyarakutaa_t*)x)->name, ((const struct kyarakutaa_t*)y)->name);
}

int N, M, K, L;
struct kyarakutaa_t kyarakutaa[114514];
char fav[114514][12];

int name_sagasu(const char* target) {
	int l = 0, r = N - 1;
	while (l <= r) {
		int m = l + (r - l) / 2;
		int c = strcmp(target, kyarakutaa[m].name);
		if (c == 0) return m;
		else if (c < 0) r = m - 1;
		else l = m + 1;
	}
	printf("ERROR: %s not found\n", target);
	exit(2);
	return 0;
}

struct kyarakutaa_data_t {
	int name_idx;
	int hyou;
};

int cmp_hyou(const void* x, const void* y) {
	const struct kyarakutaa_data_t *a = (const struct kyarakutaa_data_t*)x, *b = (const struct kyarakutaa_data_t*)y;
	if (a->hyou != b->hyou) return a->hyou > b->hyou ? -1 : 1;
	return a->name_idx < b->name_idx ? -1 : a->name_idx > b->name_idx;
}

struct kyarakutaa_data_t kyarakutaa_data[114514];
struct kyarakutaa_data_t fav_data[114514];

int dekiru(int kokokara_umeru) {
	int64_t hyou_need = 0;
	int ageta = 0;
	int i;
	int mezasu;
	for (i = 0; i < M && kokokara_umeru < K; i++) {
		if (kyarakutaa_data[kokokara_umeru].hyou < fav_data[i].hyou ||
		(kyarakutaa_data[kokokara_umeru].hyou == fav_data[i].hyou && kyarakutaa_data[kokokara_umeru].name_idx >= fav_data[i].name_idx)) kokokara_umeru++;
	}
	if (kokokara_umeru >= K) return 1;
	/* douten + namae de katu */
	mezasu = kyarakutaa_data[kokokara_umeru].hyou;
	for (i = 0; i < M; i++) {
		if (fav_data[i].hyou < mezasu && fav_data[i].name_idx < kyarakutaa_data[kokokara_umeru].name_idx) {
			hyou_need += mezasu - fav_data[i].hyou;
			ageta++;
		}
		if (ageta >= K - kokokara_umeru) break;
	}
	if (ageta >= K - kokokara_umeru && hyou_need <= L) return 1;
	/* hyou de katu */
	hyou_need = 0;
	ageta = 0;
	for (i = 0; i < M; i++) {
		if (fav_data[i].hyou < mezasu || (fav_data[i].hyou == mezasu && fav_data[i].name_idx > kyarakutaa_data[kokokara_umeru].name_idx)) {
			hyou_need += mezasu + (fav_data[i].name_idx > kyarakutaa_data[kokokara_umeru].name_idx) - fav_data[i].hyou;
			ageta++;
		}
		if (ageta >= K - kokokara_umeru) break;
	}
	if (ageta >= K - kokokara_umeru && hyou_need <= L) return 1;
	/* failed */
	return 0;
}

int main(void) {
	while (scanf("%d%d%d%d", &N, &M, &K, &L) == 4 && (N|M|K|L) != 0) {
		int i;
		for (i = 0; i < N; i++) {
			if (scanf("%11s%d", kyarakutaa[i].name, &kyarakutaa[i].x) != 2) return 1;
		}
		for (i = 0; i < M; i++) {
			if (scanf("%11s", fav[i]) != 1) return 1;
		}
		qsort(kyarakutaa, N, sizeof(*kyarakutaa), cmp_name);
		for (i = 0; i < N; i++) {
			kyarakutaa_data[i].name_idx = i;
			kyarakutaa_data[i].hyou = kyarakutaa[i].x;
		}
		for (i = 0; i < M; i++) {
			fav_data[i].name_idx = name_sagasu(fav[i]);
			fav_data[i].hyou = kyarakutaa[fav_data[i].name_idx].x;
		}
		qsort(kyarakutaa_data, N, sizeof(*kyarakutaa_data), cmp_hyou);
		qsort(fav_data, M, sizeof(*fav_data), cmp_hyou);
		if (dekiru(0)) {
			printf("%d\n", K);
		} else {
			int ng = 0, ok = K;
			while (ng + 1 < ok) {
				int mid = ng + (ok - ng) / 2;
				if (dekiru(mid)) ok = mid; else ng = mid;
			}
			printf("%d\n", K - ok);
		}
	}
	return 0;
}

