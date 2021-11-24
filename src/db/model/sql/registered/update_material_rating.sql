SELECT registered.update_material_rating(
	material_id := $1, 
	user_id := $2, 
	input_rating := $3
);