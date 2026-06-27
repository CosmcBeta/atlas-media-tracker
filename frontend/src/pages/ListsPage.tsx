import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Book, Film, Gamepad2, List, Music } from "lucide-react";
import { useState } from "react";
import { api } from "@/api/client";
import { Button } from "@/components/ui/button";
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "@/components/ui/select";

const ICONS = [
	{
		name: "list",
		label: "List",
		icon: List,
	},
	{
		name: "movie",
		label: "Movie",
		icon: Film,
	},
	{
		name: "book",
		label: "Book",
		icon: Book,
	},
	{
		name: "game",
		label: "Game",
		icon: Gamepad2,
	},
	{
		name: "music",
		label: "Music",
		icon: Music,
	},
];

function ListsPage() {
	const listsQuery = useQuery({
		queryKey: ["lists"],
		queryFn: api.getLists,
	});

	if (listsQuery.isPending) {
		return <div>Loading...</div>;
	}

	if (listsQuery.isError) {
		return <div>Error</div>;
	}

	return (
		<div>
			<CreateListButton />

			{listsQuery.data.map((list) => (
				<div key={list.id}>{list.name}</div>
			))}
		</div>
	);
}

function CreateListButton() {
	const [open, setOpen] = useState(false);
	const [name, setName] = useState("");
	const [icon, setIcon] = useState<string | null>(null);

	const queryClient = useQueryClient();

	const createList = useMutation({
		mutationFn: api.createList,
		onSuccess: () => {
			queryClient.invalidateQueries({
				queryKey: ["lists"],
			});

			setName("");
			setIcon(null);
			setOpen(false);
		},
	});

	function handleSubmit(e: React.SubmitEvent<HTMLFormElement>) {
		e.preventDefault();

		createList.mutate({
			name: name,
			icon: icon,
		});
	}

	return (
		<Dialog open={open} onOpenChange={setOpen}>
			<DialogTrigger render={<Button>Create List</Button>} />
			<DialogContent>
				<DialogHeader>
					<DialogTitle>Create New List</DialogTitle>
					<DialogDescription>You can create a new list</DialogDescription>
				</DialogHeader>
				<form onSubmit={handleSubmit}>
					<div>
						<label htmlFor="name">
							Name: <span className="text-destructive">*</span>
						</label>

						<Input
							id="name"
							value={name}
							onChange={(e) => setName(e.target.value)}
							placeholder="Watchlist"
							required
						/>
					</div>

					<div>
						<label htmlFor="icon">Icon:</label>

						<Select
							id="icon"
							value={icon ?? "none"}
							onValueChange={(value) =>
								setIcon(value === "none" ? null : value)
							}
						>
							<SelectTrigger className="w-[180px]">
								<SelectValue placeholder="Choose an icon" />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="none">No icon</SelectItem>

								{ICONS.map((item) => {
									const Icon = item.icon;

									return (
										<SelectItem key={item.name} value={item.name}>
											<div>
												<Icon size={16} />

												{item.label}
											</div>
										</SelectItem>
									);
								})}
							</SelectContent>
						</Select>
					</div>
					<DialogFooter>
						<Button type="submit" disabled={createList.isPending}>
							{createList.isPending ? "Creating..." : "Create"}
						</Button>
					</DialogFooter>
				</form>
			</DialogContent>
		</Dialog>
	);
}

// <Page>

//     <PageTitle />

//     <Toolbar />

//     <ListGrid />

// </Page>
//
// <Toolbar>
//  <SearchLists />

//  <SortDropdown />

//  <CreateListButton />
// </Toolbar>

export default ListsPage;
